// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {IChainlinkFeed, ChainlinkPriceHistory} from "../src/ChainlinkProxy.sol";

/// @title ChainlinkPriceHistory fork fuzz tests
/// @notice Fuzz the binary-search `priceAt(feed, ts)` against an independent,
///         linear-scan oracle on a live Arbitrum fork, using the BTC/USD feed.
///
/// Design note (why recent-window fuzz, not full-range):
///   A brute-force oracle that walks every aggregator round touches a distinct
///   storage slot per round. On a fork that is one RPC fetch per round on first
///   access — BTC/USD currently has ~157k rounds, so a full scan is infeasible
///   against a public RPC. Instead we:
///     * fuzz arbitrary timestamps inside a *recent round window* where the
///       linear oracle only walks a handful of rounds (cheap, fully cached), and
///     * pin deep-history correctness with deterministic exact-boundary checks at
///       a few far-back anchor rounds (no scan required — the expected round at
///       ts == round.updatedAt is known by construction).
///   The binary search is scale-invariant, so recent-window coverage generalizes.
contract ChainlinkPriceHistoryFuzzTest is Test {
    /// Arbitrum BTC/USD resolved feed (proxy on data.chain.link).
    address internal constant BTCUSD = 0x1C8a384aF067418f631683262965F7B5d1e5C788;
    /// Current aggregator phase (verified: phase 1, rounds 1..157,338).
    uint80 internal constant PHASE = 1;

    /// How many rounds behind `latest` the arbitrary-window fuzz samples.
    uint256 internal constant WINDOW_ROUNDS = 256;

    ChainlinkPriceHistory internal hist;

    uint80 internal latestAgg;
    uint80 internal latestRid;
    uint256 internal latestUpdated;
    uint256 internal firstUpdated;

    // ------------------------------------------------------------------ setup

    function setUp() public {
        string memory rpc = vm.envOr("ARB_RPC", string("https://arb1.arbitrum.io/rpc"));
        vm.createSelectFork(rpc);

        hist = new ChainlinkPriceHistory();

        (latestRid, , , latestUpdated, ) = IChainlinkFeed(BTCUSD).latestRoundData();
        latestAgg = _agg(latestRid);
        (, , , firstUpdated, ) = _round(1);

        assertEq(latestRid >> 64, PHASE, "unexpected reference phase");
        assertGt(latestAgg, WINDOW_ROUNDS, "feed too small for window");
        assertGt(latestUpdated, firstUpdated, "feed history empty");

        console.log("BTCUSD fork: phase", PHASE);
        console.log("  rounds     1..%s", latestAgg);
        console.log("  first      updatedAt %s", firstUpdated);
        console.log("  latest     updatedAt %s", latestUpdated);
        console.log("  fork block %s", block.number);
    }

    // ------------------------------------------------- arbitrary ts in window

    /// @notice For a random timestamp inside a recent [before, after] round
    ///         bracket, `priceAt` must return exactly the bracket's `before`
    ///         round — the same round the linear oracle finds. Zero discrepancy.
    function testFuzz_arbitraryTs_inRecentBracket_matchesLinearOracle(uint256 seed) public {
        unchecked {
            uint256 k = 1 + (seed % WINDOW_ROUNDS);  // rounds behind latest
            uint80 before = latestAgg - uint80(k);
            uint80 nxt    = before + 1;

            (, , , uint256 bUpd, ) = _round(before);
            (, , , uint256 aUpd, ) = _round(nxt);

            uint256 ts;
            uint80 expect;
            if (aUpd == bUpd) {
                // Same publish time on consecutive rounds: `priceAt` must pick the
                // greater aggregator round id (tie-break to the newest).
                ts = bUpd;
                expect = nxt;
            } else {
                // Target strictly inside the bracket -> the `before` round.
                // Manual clamp (vm.bound rejects raw 0 inputs): ts in [bUpd, aUpd-1].
                ts = bUpd + ((seed >> 8) % (aUpd - bUpd));
                expect = before;
            }

            ChainlinkPriceHistory.Round memory actual =
                hist.priceAt(IChainlinkFeed(BTCUSD), ts);
            ChainlinkPriceHistory.Round memory oracle = _linearOracle(ts);

            assertEq(actual.proxyRoundId, oracle.proxyRoundId, "proxyRoundId vs oracle");
            assertEq(actual.aggregatorRoundId, oracle.aggregatorRoundId, "agg round vs oracle");
            assertEq(actual.answer, oracle.answer, "answer discrepancy");
            assertEq(actual.updatedAt, oracle.updatedAt, "updatedAt vs oracle");

            assertEq(actual.aggregatorRoundId, uint64(expect), "wrong bracket round");
            assertLe(actual.updatedAt, ts, "returned a future round");
        }
    }

    /// @notice ts exactly equal to a round's updatedAt must return that round
    //          (with tie-break to the newer round when two share a timestamp).
    function testFuzz_exactRoundTimestamp_returnsThatRound(uint256 seed) public {
        uint80 a = latestAgg - uint80(seed % WINDOW_ROUNDS);
        (, , , uint256 upd, ) = _round(a);

        ChainlinkPriceHistory.Round memory actual =
            hist.priceAt(IChainlinkFeed(BTCUSD), upd);
        ChainlinkPriceHistory.Round memory oracle = _linearOracle(upd);

        assertEq(actual.proxyRoundId, oracle.proxyRoundId, "proxyRoundId vs oracle");
        assertEq(actual.answer, oracle.answer, "answer discrepancy");
        assertEq(actual.updatedAt, upd, "must land on the exact round timestamp");
    }

    // ---------------------------------------------------- deep history anchors

    /// @notice Deterministic exact-boundary checks deep in the feed's history,
    ///         where the recent-window fuzz never reaches. No full scan needed:
    ///         the expected round at ts == round.updatedAt is known by construction.
    function test_anchorRounds_deepHistory() public {
        uint80[5] memory anchors = [uint80(1), 1000, 50_000, 150_000, latestAgg];
        for (uint256 i = 0; i < anchors.length; i++) {
            uint80 a = anchors[i];
            (, , , uint256 upd, ) = _round(a);

            // (a) ts exactly at the anchor round's publish time -> returns it.
            ChainlinkPriceHistory.Round memory at =
                hist.priceAt(IChainlinkFeed(BTCUSD), upd);
            assertEq(at.aggregatorRoundId, uint64(a), "anchor round not returned at exact ts");
            assertEq(at.updatedAt, upd, "anchor updatedAt mismatch");

            // (b) one second before the anchor -> returns the previous round,
            //     which is the strictly-earlier publish (monotonicity).
            if (a == 1) {
                if (upd > firstUpdated) {
                    vm.expectRevert(
                        abi.encodeWithSelector(ChainlinkPriceHistory.BeforeHistory.selector, upd - 1)
                    );
                    hist.priceAt(IChainlinkFeed(BTCUSD), upd - 1);
                }
            } else {
                ChainlinkPriceHistory.Round memory before =
                    hist.priceAt(IChainlinkFeed(BTCUSD), upd - 1);
                // Must be the previous aggregator round (no gap below a real round).
                assertEq(before.aggregatorRoundId, uint64(a - 1), "before-anchor round mismatch");
                assertLt(before.updatedAt, upd, "monotonicity violated before-anchor");
            }
        }
    }

    /// @notice The feed's known history span: latest must equal what priceAt
    ///         returns for block.timestamp (the fork's current time).
    function test_latestRoundIsReturnedForNow() public {
        ChainlinkPriceHistory.Round memory r =
            hist.priceAt(IChainlinkFeed(BTCUSD), block.timestamp);
        assertEq(r.proxyRoundId, latestRid, "latest round not returned for now");
        assertEq(r.updatedAt, latestUpdated, "latest updatedAt mismatch");
    }

    /// @notice A target older than the first published round reverts BeforeHistory.
    function test_beforeHistoryReverts() public {
        vm.expectRevert(
            abi.encodeWithSelector(
                ChainlinkPriceHistory.BeforeHistory.selector,
                firstUpdated - 1
            )
        );
        hist.priceAt(IChainlinkFeed(BTCUSD), firstUpdated - 1);
    }

    // ------------------------------------------------------- discrepancy report

    /// @notice Non-asserting diagnostic: sample the recent window and report how
    ///         stale the returned price is (ts - returned.updatedAt). Gives a
    ///         concrete number for "the discrepancy" — the heartbeat-quantized
    ///         staleness is inherent to Chainlink, not a bug. (0 vs oracle is
    ///         asserted by the fuzz tests above, so no oracle is run here.)
    function test_statistics_reportedStalenessInWindow() public {
        uint256 x = 0x9e3779b97f4a7c15; // golden-ratio seed, deterministic single run
        uint256 n = 256; // keep one tx under the gas limit (~0.5 Mgas/sample)
        uint256 maxDelta = 0;
        uint256 sumDelta = 0;
        for (uint256 i = 0; i < n; i++) {
            unchecked {
                x = (x * 1103515245 + 12345) & ((1 << 48) - 1); // LCG, cheap
            }

            uint80 before = latestAgg - uint80(1 + (x % WINDOW_ROUNDS));
            (, , , uint256 bUpd, ) = _round(before);
            (, , , uint256 aUpd, ) = _round(before + 1);

            uint256 ts = bUpd + ((x >> 8) % (aUpd - bUpd + 1));
            if (ts > aUpd) ts = aUpd;

            ChainlinkPriceHistory.Round memory r =
                hist.priceAt(IChainlinkFeed(BTCUSD), ts);

            uint256 delta = ts > r.updatedAt ? ts - r.updatedAt : 0;
            if (delta > maxDelta) maxDelta = delta;
            sumDelta += delta;
        }

        console.log("samples           %s", n);
        console.log("price staleness max  %s s (~%s min)", maxDelta, maxDelta / 60);
        console.log("price staleness mean %s s", sumDelta / n);
    }

    // --------------------------------------------------------------- helpers

    function _agg(uint80 rid) internal pure returns (uint80) {
        return rid & uint80((1 << 64) - 1);
    }

    function _rid(uint80 agg) internal pure returns (uint80) {
        return (PHASE << 64) | agg;
    }

    function _round(uint80 agg)
        internal
        view
        returns (uint80, int256, uint256, uint256, uint80)
    {
        return IChainlinkFeed(BTCUSD).getRoundData(_rid(agg));
    }

    /// Independent linear-scan oracle: greatest aggregator round (top-down) whose
    /// updatedAt is non-zero and <= ts. This is the "ground truth" the binary
    /// search in `priceAt` must reproduce exactly.
    function _linearOracle(uint256 ts)
        internal
        view
        returns (ChainlinkPriceHistory.Round memory r)
    {
        for (uint80 a = latestAgg; a > 0; --a) {
            (, int256 answer, uint256 startedAt, uint256 updatedAt, ) = _round(a);
            if (updatedAt != 0 && updatedAt <= ts) {
                r.proxyRoundId = _rid(a);
                r.phase = uint16(PHASE);
                r.aggregatorRoundId = uint64(a);
                r.answer = answer;
                r.startedAt = startedAt;
                r.updatedAt = updatedAt;
                return r;
            }
        }
        revert("linear oracle: ts before all rounds");
    }
}