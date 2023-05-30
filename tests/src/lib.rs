#![cfg(test)]
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]

use iter_all::IterAll;
use iter_all_proc::IterAll;

#[derive(Default)]
pub struct Endpoint<const NAME: &'static str, In, Out> {
    _p: Option<(In, Out)>,
}

impl<const NAME: &'static str, In, Out> Endpoint<NAME, In, Out> {
    const fn default() -> Self {
        Self { _p: None }
    }

    const fn name(&self) -> &'static str {
        NAME
    }

    pub const fn req(&self) -> Endpoint<NAME, In, Out> {
        Endpoint::<NAME, In, Out>::default()
    }
}

#[derive(IterAll)]
pub enum TransactionEndpoints {
    Add(Endpoint<"add_transaction", i32, i32>),
    Get(Endpoint<"get_transactions", i32, i32>),
    Delete(Endpoint<"delete_transaction", i32, i32>),
    Edit(Endpoint<"edit_transaction", i32, i32>),
}

#[test]
fn test() {
    let mut sargovnost = vec![];

    TransactionEndpoints::iter_all(|tx| match tx {
        TransactionEndpoints::Add(add) => {
            sargovnost.push(add.name());
        }
        TransactionEndpoints::Get(add) => {
            sargovnost.push(add.name());
        }
        TransactionEndpoints::Delete(add) => {
            sargovnost.push(add.name());
        }

        TransactionEndpoints::Edit(add) => {
            sargovnost.push(add.name());
        }
    });

    assert_eq!(
        sargovnost,
        vec![
            "add_transaction",
            "get_transactions",
            "delete_transaction",
            "edit_transaction",
        ]
    );

    assert_eq!(
        vec![
            TransactionEndpoints::add().name(),
            TransactionEndpoints::get().name(),
            TransactionEndpoints::delete().name(),
            TransactionEndpoints::edit().name(),
        ],
        vec![
            "add_transaction",
            "get_transactions",
            "delete_transaction",
            "edit_transaction",
        ]
    )
}
