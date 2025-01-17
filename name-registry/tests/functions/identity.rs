mod success {
    use crate::utils::{
        abi::{identity, register, set_identity},
        setup, REGISTER_DURATION,
    };
    use fuels::prelude::*;

    #[tokio::test]
    async fn can_get_identity() {
        let (instance, acc1, wallet2) = setup().await;
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        register(
            &instance,
            &acc1.name,
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
        )
        .await;

        let previous_identity = identity(&instance, &acc1.name).await;

        assert_eq!(previous_identity.0.value.unwrap(), acc1.identity());

        set_identity(&instance, &acc1.name, wallet_identity2.clone()).await;

        let new_identity = identity(&instance, &acc1.name).await;

        assert_eq!(new_identity.0.value.unwrap(), wallet_identity2);
    }
}

mod revert {
    use crate::utils::{abi::identity, setup};

    #[tokio::test]
    #[should_panic(expected = "`Result::unwrap()` on an `Err` value")]
    async fn cant_get_identity_when_not_registered() {
        let (instance, acc, _wallet2) = setup().await;

        let identity = identity(&instance, &acc.name).await;
        identity.0.value.unwrap();
    }
}
