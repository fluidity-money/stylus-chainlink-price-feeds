#![no_std]

#[cfg(feature = "bobcat-sdk")]
use bobcat_maths::U;

#[allow(unused)]
use bobcat_interfaces::chainlink_price_feed::SEL_LATEST_ROUND_DATA;

#[cfg(feature = "bobcat-sdk")]
use bobcat_call::static_call_word;

#[cfg(feature = "stylus-sdk")]
extern crate alloc;

#[cfg(feature = "stylus-sdk")]
use alloc::vec::Vec;

#[cfg(feature = "stylus-sdk")]
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    call::static_call,
    prelude::{Host, StaticCallContext, calls::errors::Error as StylusError},
};

#[cfg(feature = "stylus-sdk")]
use core::fmt::{Display, Formatter, Result as FmtResult};

pub trait ChainlinkPriceFeed {
    fn addr(self) -> [u8; 20];
    fn decimals(self) -> u8;
}

pub mod robinhood {
    use bobcat_cd::address;
    use crate::ChainlinkPriceFeed;

    #[cfg(feature = "stylus-sdk")]
    use stylus_sdk::alloy_primitives::Address;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Robinhood {
        BtcUsd,
        EthUsd,
        LinkUsd,
        BtcBUsd,
        CbbtcUsd,
        LbtcUsd,
        WbtcUsd,
        UsdcUsd,
        UsdtUsd,
        UsdsUsd,
        UsdeUsd,
        UsdgUsd,
        EurcUsd,
        EnaUsd,
        WeethUsd,
        WeethEeth,
        WstethUsd,
        WstethSteth,
        SyrupusdcUsd,
        SyrupusdcUsdc,
        SyrupusdtUsdt,
        SyrupUsdgUsdg,
        AaplUsd,
        AmdUsd,
        AmznUsd,
        AsmlUsd,
        BabaUsd,
        ClskUsd,
        CoinUsd,
        CrclUsd,
        CrwvUsd,
        DellUsd,
        EwyUsd,
        GmeUsd,
        GooglUsd,
        IntcUsd,
        IonqUsd,
        MetaUsd,
        MsftUsd,
        MstrUsd,
        MuUsd,
        NbisUsd,
        NvdaUsd,
        OrclUsd,
        PltrUsd,
        QqqUsd,
        RgtiUsd,
        RklbUsd,
        SgovUsd,
        SlvUsd,
        SndkUsd,
        SpcxUsd,
        SpyUsd,
        TslaUsd,
        TsmUsd,
        UsarUsd,
        UsoUsd,
    }

    impl ChainlinkPriceFeed for Robinhood {
        fn addr(self) -> [u8; 20] {
            match self {
                Self::BtcUsd => address!(b"a2c5184bF03d373Dc9dE4876eb4Bce595B460251"),
                Self::EthUsd => address!(b"78F3556b67E17Df817D51Ef5a990cDaF09E8d3A9"),
                Self::LinkUsd => address!(b"e86e3422Aa9B5e8ee9f3E41a63975bC387A8bce9"),
                Self::BtcBUsd => address!(b"5BB5e6a17a477d5B6Fec77b4322daD4A66bFb732"),
                Self::CbbtcUsd => address!(b"0009cD492adf8167f9eEBf1293556A673530a21a"),
                Self::LbtcUsd => address!(b"a621344AdAEE699491597Fd8890E0C59a5BFBE59"),
                Self::WbtcUsd => address!(b"62107b0d3adA75fc1697fD342d99eed947a3aA5E"),
                Self::UsdcUsd => address!(b"9e6f4605992a899eE2999999F3Ec80C41F452546"),
                Self::UsdtUsd => address!(b"bf3550B6fAe1671da7C238Af12e03Ac586BEf3B1"),
                Self::UsdsUsd => address!(b"2D88D75b625633dCcd65d9d53BfDD3Aea2d8e84f"),
                Self::UsdeUsd => address!(b"b9fB4e65744E4178894f7C61CF80E8a48A5f224a"),
                Self::UsdgUsd => address!(b"61B7e5650328764B076A108EFF5fa7282a1B9aD2"),
                Self::EurcUsd => address!(b"fF2B10c1973eD10c841434f98e456d8f3a0D7DD8"),
                Self::EnaUsd => address!(b"2A291496b3aa19d8948e442Ef28Ee952f3Ee97E8"),
                Self::WeethUsd => address!(b"f882e1D50352aecB0Ac85378378918BCf40511e7"),
                Self::WeethEeth => address!(b"b63f44E40aA811Cc69Fc55da786a5F3834100B4A"),
                Self::WstethUsd => address!(b"3F5040B50FB37934573B210fE54B53a6F1A792E8"),
                Self::WstethSteth => address!(b"8E3Eb706B170c8FD1DdcD402932D952887736f9A"),
                Self::SyrupusdcUsd => address!(b"8765c3B9Cda41d1029E780D0c1C37C8200DC4675"),
                Self::SyrupusdcUsdc => address!(b"6317f016FA3e312C4625dee51d32b43a223011f8"),
                Self::SyrupusdtUsdt => address!(b"BB688c0184Ce03fEdac89D71ccE752Ab21bC2999"),
                Self::SyrupUsdgUsdg => address!(b"Dd194C66aDcb422F188a04434e4824D70c151cF0"),
                Self::AaplUsd => address!(b"6B22A786bAa607d76728168703a39Ea9C99f2cD0"),
                Self::AmdUsd => address!(b"943A29E7ae51A4798823ca9eEd2ed533B2A22C72"),
                Self::AmznUsd => address!(b"D5a1508ceD74c084eBf3cBe853e2C968fB2a651C"),
                Self::AsmlUsd => address!(b"B4106147E8cce40b7d46124090d373A71b70f87D"),
                Self::BabaUsd => address!(b"62Cc8F9b5f56a33c9C8A60c8B92779f523c4E984"),
                Self::ClskUsd => address!(b"810c12D3a554Bc47fd39597Fe3b3AAC4941F50eF"),
                Self::CoinUsd => address!(b"A3a468A452940B7D6b69991207B508c609a98Ef2"),
                Self::CrclUsd => address!(b"6652eDf64bA3731C4F2D3ce821A0Fb1f1f6b482a"),
                Self::CrwvUsd => address!(b"e1b3aABCAFAd1c94708dc1367dcfF8Aa4407487C"),
                Self::DellUsd => address!(b"1C6c8cADBe02E19129c39dDB92281cE4c0bf206b"),
                Self::EwyUsd => address!(b"EFdf54610B62A7753Ec30bDc380847c12D32e1D1"),
                Self::GmeUsd => address!(b"27C71df6A64fB476468EdF256CF72c038baB5B67"),
                Self::GooglUsd => address!(b"F6f373a037c30F0e5010d854385cA89185AE638b"),
                Self::IntcUsd => address!(b"3f390C5C24628Ac7C489515402235FeAD71D1913"),
                Self::IonqUsd => address!(b"22EfeC4919baf55F360E0EDee4AbEB26DE4971eb"),
                Self::MetaUsd => address!(b"7C38C00C30BEe9378381E7B6135d7283356D71b1"),
                Self::MsftUsd => address!(b"45C3C877C15E6BA2EBB19eA114Ea508d14C1Af2E"),
                Self::MstrUsd => address!(b"396118bdFB181e6240E74D243F266B061c0edc3D"),
                Self::MuUsd => address!(b"425EEFdCf05ed6526C3cE61Af99429A228a6d596"),
                Self::NbisUsd => address!(b"E1D87B116Ba0fe898998f1D140339D1fA1E09705"),
                Self::NvdaUsd => address!(b"379EC4f7C378F34a1B47E4F3cbeBCbAC3E8E9F15"),
                Self::OrclUsd => address!(b"0e6a64a2B58A6693a531E6c555f3A5d042eEA844"),
                Self::PltrUsd => address!(b"820ABedFF239034956B7A9d2F0a331f9F075eB4c"),
                Self::QqqUsd => address!(b"80901d846d5D7B030F26B480776EE3b29374C2ae"),
                Self::RgtiUsd => address!(b"2A045cF1C49c61c166C036d2f06FA2D2d984f765"),
                Self::RklbUsd => address!(b"045477BF65Aef6f4F2386ad0164579e48381CC74"),
                Self::SgovUsd => address!(b"a0DF4ee0fFf975306345875E3548Fcc519577A11"),
                Self::SlvUsd => address!(b"209b73908e92Ae021826eD79609845451Ecba2ce"),
                Self::SndkUsd => address!(b"fb133Fa4B7b385802B693a293606682Df47109A3"),
                Self::SpcxUsd => address!(b"B265810950ba6c5C0Ff821c9963014a56fD8Bffb"),
                Self::SpyUsd => address!(b"319724394D3A0e3669269846abE664Cd621f9f6A"),
                Self::TslaUsd => address!(b"4A1166a659A55625345e9515b32adECea5547C38"),
                Self::TsmUsd => address!(b"874cF94aa8eC88Fd9560094dD065f2fB3E41Fc2F"),
                Self::UsarUsd => address!(b"A994d3684e8400A6c8078226925779FdeE682DD9"),
                Self::UsoUsd => address!(b"75a9c76Ef439e2C7c2E5a34Ab105EcFe3766431c"),
            }
        }

        fn decimals(self) -> u8 {
            match self {
                Self::BtcUsd
                | Self::EthUsd
                | Self::LinkUsd
                | Self::BtcBUsd
                | Self::CbbtcUsd
                | Self::LbtcUsd
                | Self::WbtcUsd
                | Self::UsdcUsd
                | Self::UsdtUsd
                | Self::UsdsUsd
                | Self::UsdeUsd
                | Self::UsdgUsd
                | Self::EurcUsd
                | Self::EnaUsd
                | Self::WeethUsd
                | Self::WstethUsd
                | Self::SyrupusdcUsd
                | Self::AaplUsd
                | Self::AmdUsd
                | Self::AmznUsd
                | Self::AsmlUsd
                | Self::BabaUsd
                | Self::ClskUsd
                | Self::CoinUsd
                | Self::CrclUsd
                | Self::CrwvUsd
                | Self::DellUsd
                | Self::EwyUsd
                | Self::GmeUsd
                | Self::GooglUsd
                | Self::IntcUsd
                | Self::IonqUsd
                | Self::MetaUsd
                | Self::MsftUsd
                | Self::MstrUsd
                | Self::MuUsd
                | Self::NbisUsd
                | Self::NvdaUsd
                | Self::OrclUsd
                | Self::PltrUsd
                | Self::QqqUsd
                | Self::RgtiUsd
                | Self::RklbUsd
                | Self::SgovUsd
                | Self::SlvUsd
                | Self::SndkUsd
                | Self::SpcxUsd
                | Self::SpyUsd
                | Self::TslaUsd
                | Self::TsmUsd
                | Self::UsarUsd
                | Self::UsoUsd => 8,
                Self::WeethEeth
                | Self::WstethSteth
                | Self::SyrupusdcUsdc
                | Self::SyrupusdtUsdt
                | Self::SyrupUsdgUsdg => 18,
            }
        }
    }

    #[cfg(feature = "stylus-sdk")]
    impl From<Robinhood> for [u8; 20] {
        fn from(x: Robinhood) -> Self {
            x.addr()
        }
    }

    #[cfg(feature = "stylus-sdk")]
    impl Into<Address> for Robinhood {
        fn into(self) -> Address {
            Address::new(self.addr())
        }
    }
}

pub mod arbitrum {
    use bobcat_cd::address;
    use crate::ChainlinkPriceFeed;

    #[cfg(feature = "stylus-sdk")]
    use stylus_sdk::alloy_primitives::Address;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Arbitrum {
        // 8-decimal crypto pairs
        BtcUsd,
        EthUsd,
        LinkUsd,
        ArbUsd,
        UsdcUsd,
        UsdtUsd,
        DaiUsd,
        WbtcUsd,
        FraxUsd,
        GmxUsd,
        UsdsUsd,
        UsdeUsd,
        UsdgUsd,
        EnaUsd,
        CrvUsd,
        AaveUsd,
        // 18-decimal wrapped/derivative ETH + stablecoin pairs
        StethEth,
        CbethEth,
        WeethEth,
        WstethEth,
        ReethEth,
        RsethEth,
        EzethEth,
        EurcUsd,
        // 8-decimal equities
        AaplUsd,
        AmznUsd,
        CoinUsd,
        GooglUsd,
        MetaUsd,
        MsftUsd,
        NvdaUsd,
        SpyUsd,
        TslaUsd,
        // 8-decimal forex
        EurUsd,
        GbpUsd,
        AudUsd,
        CadUsd,
        ChfUsd,
        JpyUsd,
    }

    impl ChainlinkPriceFeed for Arbitrum {
        fn addr(self) -> [u8; 20] {
            match self {
                Self::BtcUsd => address!(b"6ce185860a4963106506C203335A2910413708e9"),
                Self::EthUsd => address!(b"639Fe6ab55C921f74e7fac1ee960C0B6293ba612"),
                Self::LinkUsd => address!(b"86E53CF1B870786351Da77A57575e79CB55812CB"),
                Self::ArbUsd => address!(b"b2A824043730FE05F3DA2efaFa1CBbe83fa548D6"),
                Self::UsdcUsd => address!(b"50834F3163758fcC1Df9973b6e91f0F0F0434aD3"),
                Self::UsdtUsd => address!(b"3f3f5dF88dC9F13eac63DF89EC16ef6e7E25DdE7"),
                Self::DaiUsd => address!(b"c5C8E77B397E531B8EC06BFb0048328B30E9eCfB"),
                Self::WbtcUsd => address!(b"d0C7101eACbB49F3deCcCc166d238410D6D46d57"),
                Self::FraxUsd => address!(b"6AB8414a33065d4889049F6EBd202F1cb5e45537"),
                Self::GmxUsd => address!(b"DB98056FecFff59D032aB628337A4887110df3dB"),
                Self::UsdsUsd => address!(b"37833E5b3fbbEd4D613a3e0C354eF91A42B81eeB"),
                Self::UsdeUsd => address!(b"88AC7Bca36567525A866138F03a6F6844868E0Bc"),
                Self::UsdgUsd => address!(b"E9Bd4Ca2CDacb56cd06c9C4C5231e69c33340B3c"),
                Self::EnaUsd => address!(b"9eE96caa9972c801058CAA8E23419fc6516FbF7e"),
                Self::CrvUsd => address!(b"aebDA2c976cfd1eE1977Eac079B4382acb849325"),
                Self::AaveUsd => address!(b"aD1d5344AaDE45F43E596773Bcc4c423EAbdD034"),
                Self::StethEth => address!(b"ded2c52b75B24732e9107377B7Ba93eC1fFa4BAf"),
                Self::CbethEth => address!(b"a668682974E3f121185a3cD94f00322beC674275"),
                Self::WeethEth => address!(b"E141425bc1594b8039De6390db1cDaf4397EA22b"),
                Self::WstethEth => address!(b"b523AE262D20A936BC152e6023996e46FDC2A95D"),
                Self::ReethEth => address!(b"D6aB2298946840262FcC278fF31516D39fF611eF"),
                Self::RsethEth => address!(b"b0EA543f9F8d4B818550365d13F66Da747e1476A"),
                Self::EzethEth => address!(b"11E1836bFF2ce9d6A5bec9cA79dc998210f3886d"),
                Self::EurcUsd => address!(b"dE85Ee1a7D45B2B3c56aCb1B8A43EdEEfBa50cB8"),
                Self::AaplUsd => address!(b"8d0CC5f38f9E802475f2CFf4F9fc7000C2E1557c"),
                Self::AmznUsd => address!(b"d6a77691f071E98Df7217BED98f38ae6d2313EBA"),
                Self::CoinUsd => address!(b"950DC95D4E537A14283059bADC2734977C454498"),
                Self::GooglUsd => address!(b"1D1a83331e9D255EB1Aaf75026B60dFD00A252ba"),
                Self::MetaUsd => address!(b"cd1bd86fDc33080DCF1b5715B6FCe04eC6F85845"),
                Self::MsftUsd => address!(b"Dde33fb9F21739602806580bdd73BAd831DcA867"),
                Self::NvdaUsd => address!(b"4881A4418b5F2460B21d6F08CD5aA0678a7f262F"),
                Self::SpyUsd => address!(b"46306F3795342117721D8DEd50fbcF6DF2b3cc10"),
                Self::TslaUsd => address!(b"3609baAa0a9b1f0FE4d6CC01884585d0e191C3E3"),
                Self::EurUsd => address!(b"A14d53bC1F1c0F31B4aA3BD109344E5009051a84"),
                Self::GbpUsd => address!(b"9C4424Fd84C6661F97D8d6b3fc3C1aAc2BeDd137"),
                Self::AudUsd => address!(b"9854e9a850e7C354c1de177eA953a6b1fba8Fc22"),
                Self::CadUsd => address!(b"f6DA27749484843c4F02f5Ad1378ceE723dD61d4"),
                Self::ChfUsd => address!(b"e32AccC8c4eC03F6E75bd3621BfC9Fbb234E1FC3"),
                Self::JpyUsd => address!(b"3dD6e51CB9caE717d5a8778CF79A04029f9cFDF8"),
            }
        }

        fn decimals(self) -> u8 {
            match self {
                Self::BtcUsd
                | Self::EthUsd
                | Self::LinkUsd
                | Self::ArbUsd
                | Self::UsdcUsd
                | Self::UsdtUsd
                | Self::DaiUsd
                | Self::WbtcUsd
                | Self::FraxUsd
                | Self::GmxUsd
                | Self::UsdsUsd
                | Self::UsdeUsd
                | Self::UsdgUsd
                | Self::EnaUsd
                | Self::CrvUsd
                | Self::AaveUsd
                | Self::AaplUsd
                | Self::AmznUsd
                | Self::CoinUsd
                | Self::GooglUsd
                | Self::MetaUsd
                | Self::MsftUsd
                | Self::NvdaUsd
                | Self::SpyUsd
                | Self::TslaUsd
                | Self::EurUsd
                | Self::GbpUsd
                | Self::AudUsd
                | Self::CadUsd
                | Self::ChfUsd
                | Self::JpyUsd => 8,
                Self::StethEth
                | Self::CbethEth
                | Self::WeethEth
                | Self::WstethEth
                | Self::ReethEth
                | Self::RsethEth
                | Self::EzethEth
                | Self::EurcUsd => 18,
            }
        }
    }

    #[cfg(feature = "stylus-sdk")]
    impl From<Arbitrum> for [u8; 20] {
        fn from(x: Arbitrum) -> Self {
            x.addr()
        }
    }

    #[cfg(feature = "stylus-sdk")]
    impl Into<Address> for Arbitrum {
        fn into(self) -> Address {
            Address::new(self.addr())
        }
    }
}

#[cfg(feature = "stylus-sdk")]
#[derive(Debug, Clone, PartialEq)]
pub enum ErrGetLatestRoundDataReason {
    Revert,
    BadRd,
}

/// Error if we didn't get a good result from Chainlink. True if we had problems decoding a
/// valid reply. Returns the calldata in that situation.
#[cfg(feature = "stylus-sdk")]
#[derive(Debug, Clone, PartialEq)]
pub struct ErrGetLatestRoundData(pub Vec<u8>, pub ErrGetLatestRoundDataReason);

#[cfg(feature = "stylus-sdk")]
impl Display for ErrGetLatestRoundData {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "stylus-sdk")]
impl core::error::Error for ErrGetLatestRoundData {}

#[cfg(feature = "stylus-sdk")]
pub fn get_latest_round_data<H, C, P>(
    host: &H,
    ctx: C,
    p: P,
) -> Result<U256, ErrGetLatestRoundData>
where
    H: Host + ?Sized,
    C: StaticCallContext,
    P: ChainlinkPriceFeed,
{
    let rd = static_call(host, ctx, Address::new(p.addr()), &SEL_LATEST_ROUND_DATA).map_err(|v| match v {
        StylusError::Revert(v) => ErrGetLatestRoundData(v, ErrGetLatestRoundDataReason::Revert),
        _ => unimplemented!(),
    })?;
    U256::try_from_be_slice(&rd).ok_or(ErrGetLatestRoundData(
        rd,
        ErrGetLatestRoundDataReason::BadRd,
    ))
}

#[cfg(feature = "bobcat-sdk")]
pub fn get_latest_round_data_bool<P: ChainlinkPriceFeed>(p: P) -> (bool, U) {
    static_call_word(p.addr(), &SEL_LATEST_ROUND_DATA, u64::MAX, 0)
}

#[cfg(feature = "bobcat-sdk")]
pub fn get_latest_round_data_opt<P: ChainlinkPriceFeed>(p: P) -> Option<U> {
    if let (true, p) = get_latest_round_data_bool(p) {
        Some(p)
    } else {
        None
    }
}