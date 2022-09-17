use tokio_postgres::Transaction;

pub trait Dao {
    fn tran(&self) -> &Transaction;
}

pub struct BaseDao<'a> {
    tran: &'a Transaction<'a>,
}

impl Dao for BaseDao<'_> {
    fn tran(&self) -> &Transaction {
        self.tran
    }
}

impl<'a> BaseDao<'a> {
    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            tran
        }
    }
}