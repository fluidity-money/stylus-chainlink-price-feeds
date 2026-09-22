// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

interface IChainlinkFeed {
    // RoundId encoding on live proxies: proxyRoundId = (phase << 64) | aggregatorRoundId.
    // Phases increment every time Chainlink swaps the underlying aggregator, so the
    // value is NOT a plain counter and has ~2^64 holes between phases.
    function latestRoundData()
        external
        view
        returns (
            uint80 roundId,
            int256 answer,
            uint256 startedAt,
            uint256 updatedAt,
            uint80 answeredInRound
        );

    // Within one phase, aggregatorRoundId is contiguous 1..N and updatedAt is
    // monotonically non-decreasing with it. That monotonicity is what lets us
    // binary search for the round live at a target timestamp.
    function getRoundData(uint80 _roundId)
        external
        view
        returns (
            uint80 roundId,
            int256 answer,
            uint256 startedAt,
            uint256 updatedAt,
            uint80 answeredInRound
        );
}

/// @title ChainlinkPriceHistory
/// @notice Find the price feed round active at (or nearest before) a unix timestamp,
///         using only the feed proxy address and the timestamp.
/// @dev   Pure read helper — no state, no LINK, no per-round registration. Callers
///        pass the standard AggregatorV3 PROXY address (the address on data.chain.link).
contract ChainlinkPriceHistory {
    struct Round {
        uint80 proxyRoundId;      // the value you'd pass to getRoundData — this is "the round id"
        uint16 phase;             // phase component (proxyRoundId >> 64)
        uint64 aggregatorRoundId; // round within the phase (uint64(proxyRoundId))
        int256 answer;            // raw scaled answer (divide by decimals() to unscale)
        uint256 startedAt;        // when the round started
        uint256 updatedAt;        // when the answer was published — compare against target
    }

    error BeforeHistory(uint64 requestedAt);

    /// @notice Return the round whose answer was published at or before `target`.
    /// @dev    Handles aggregator-upgrade phase jumps by walking phases backwards.
    /// @param  feed   The chainlink AggregatorV3 proxy address.
    /// @param  target Unix timestamp (seconds).
    /// @return round  The active round: proxyRoundId, price, and timestamps.
    function priceAt(IChainlinkFeed feed, uint64 target)
        external
        view
        returns (Round memory round)
    {
        (uint80 latestId, , , , ) = feed.latestRoundData();
        uint16 curPhase = uint16(latestId >> 64);
        uint64 curAgg   = uint64(latestId);

        // Walk from the newest phase down to the oldest.
        for (uint32 p = curPhase; p > 0; --p) {
            uint16 phase = uint16(p);

            // (roundId, answer, startedAt, updatedAt, answeredInRound) — all decoded
            // via a safe low-level read that returns zeros on revert, because legacy
            // pre-64-bit phases revert getRoundData instead of returning zero.
            (, , , uint256 firstUpdated, ) = _readRound(feed, phase, 1);
            if (firstUpdated == 0) continue;              // unreachable / no data, skip
            if (target < firstUpdated) continue;          // this phase begins after target

            // Upper bound on the round to search within this phase.
            uint64 lastAgg = (p == curPhase) ? curAgg : _lastAggregatorRound(feed, phase);

            // Binary search: largest aggregatorRoundId with updatedAt <= target.
            uint64 lo = 1;
            uint64 hi = lastAgg;
            while (lo < hi) {
                uint64 mid = lo + (hi - lo + 1) / 2;
                (, , , uint256 upd, ) = _readRound(feed, phase, mid);
                if (upd != 0 && upd <= target) lo = mid;
                else hi = mid - 1;
            }

            (, int256 answer, uint256 startedAt, uint256 updatedAt, ) =
                _readRound(feed, phase, lo);
            if (updatedAt > target) revert BeforeHistory(target); // target older than feed start

            return Round({
                proxyRoundId: _roundId(phase, lo),
                phase: phase,
                aggregatorRoundId: lo,
                answer: answer,
                startedAt: startedAt,
                updatedAt: updatedAt
            });
        }
        revert BeforeHistory(target);
    }

    /// @notice Read getRoundData without bricking the caller when a round is missing.
    ///         Returns zeros if the round (or the whole phase) reverts.
    function _readRound(IChainlinkFeed feed, uint16 phase, uint64 agg)
        private
        view
        returns (
            uint80 roundId,
            int256 answer,
            uint256 startedAt,
            uint256 updatedAt,
            uint80 answeredInRound
        )
    {
        (bool ok, bytes memory ret) = address(feed).staticcall(
            abi.encodeWithSelector(
                IChainlinkFeed.getRoundData.selector,
                _roundId(phase, agg)
            )
        );
        if (!ok || ret.length < 160) return (0, 0, 0, 0, 0);
        return abi.decode(ret, (uint80, int256, uint256, uint256, uint80));
    }

    /// @notice True if the requested round exists and has a stored answer.
    function _hasData(IChainlinkFeed feed, uint16 phase, uint64 agg)
        private
        view
        returns (bool)
    {
        (, , , uint256 updatedAt, ) = _readRound(feed, phase, agg);
        return updatedAt != 0;
    }

    /// @notice Find the last valid aggregatorRoundId of a non-current phase.
    /// @dev   On reachable phases the proxy does NOT revert for out-of-range rounds:
    ///        it returns a zero-blob (updatedAt == 0). So the phase boundary is the
    ///        last round with updatedAt != 0. Gallop up powers of two until we hit a
    ///        zero, then binary search the boundary.
    function _lastAggregatorRound(IChainlinkFeed feed, uint16 phase)
        private
        view
        returns (uint64)
    {
        uint64 lo = 1;
        uint64 hi = 1;
        while (_hasData(feed, phase, hi)) {
            lo = hi;
            if (hi > type(uint64).max / 2) break;
            hi *= 2;
        }
        while (lo + 1 < hi) {
            uint64 mid = lo + (hi - lo) / 2;
            if (_hasData(feed, phase, mid)) lo = mid;
            else hi = mid;
        }
        return lo;
    }

    /// @notice Encode a (phase, aggregatorRoundId) pair into a proxy roundId.
    function _roundId(uint16 phase, uint64 agg) private pure returns (uint80) {
        return (uint80(phase) << 64) | uint80(agg);
    }
}
