use iter_all::IterAll;
use iter_all_proc::IterAll;

#[derive(Default)]
pub struct Endpoint<const NAME: &'static str, In, Out> {
    _p: Option<(In, Out)>,
}

impl<const NAME: &'static str, In, Out> Endpoint<NAME, In, Out> {
    const fn new() -> Self {
        Self { _p: None }
    }

    const fn name(&self) -> &'static str {
        NAME
    }

    const fn req(&self) -> Endpoint<NAME, In, Out> {
        Endpoint::<NAME, In, Out>::new()
    }
}
#[derive(Default)]
struct Wallet;

#[derive(IterAll)]
enum TransactionEndpoints {
    Add(Endpoint<"add_transaction", Wallet, Wallet>),
    Get(Endpoint<"get_transactions", i32, Vec<Wallet>>),
    Delete(Endpoint<"delete_transaction", i32, i32>),
    Edit(Endpoint<"edit_transaction", i32, i32>),
}

static EDIT: Endpoint<"edit_transaction", i32, i32> = TransactionEndpoints::EDIT;
static EDIT_REQ: Endpoint<"edit_transaction", i32, i32> = TransactionEndpoints::EDIT.req();

#[test]
fn test_with_generics() {
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
            TransactionEndpoints::ADD.name(),
            TransactionEndpoints::GET.name(),
            TransactionEndpoints::DELETE.name(),
            TransactionEndpoints::EDIT.name(),
        ],
        vec![
            "add_transaction",
            "get_transactions",
            "delete_transaction",
            "edit_transaction",
        ]
    );

    assert_eq!(EDIT.name(), "edit_transaction");
    assert_eq!(EDIT_REQ.name(), "edit_transaction");
}
