use crate::impls::one_for_all::chain::OfaChainContext;
use crate::impls::one_for_all::relay::OfaRelayContext;
use crate::traits::chain_context::{ChainContext, IbcChainContext};
use crate::traits::one_for_all::chain::OfaChain;
use crate::traits::one_for_all::relay::OfaRelay;
use crate::traits::relay_context::RelayContext;

pub fn relay_context<Relay: OfaRelay>(
    relay: Relay,
    src_chain: Relay::SrcChain,
    dst_chain: Relay::DstChain,
    src_client_id: <Relay::SrcChain as OfaChain>::ClientId,
    dst_client_id: <Relay::DstChain as OfaChain>::ClientId,
    runtime: Relay::Runtime,
) -> impl RelayContext {
    OfaRelayContext::new(
        relay,
        src_chain,
        dst_chain,
        src_client_id,
        dst_client_id,
        runtime,
    )
}

pub fn chain_context<Chain: OfaChain>(chain: Chain, runtime: Chain::Runtime) -> impl ChainContext {
    OfaChainContext::new(chain, runtime)
}

pub fn ibc_chain_context<Chain, Counterparty>(
    chain: Chain,
    runtime: Chain::Runtime,
) -> impl IbcChainContext<Counterparty>
where
    Chain: OfaChain,
    Counterparty: ChainContext<Height = Chain::CounterpartyHeight>,
{
    OfaChainContext::new(chain, runtime)
}
