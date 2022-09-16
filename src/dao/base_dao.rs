use tokio_postgres::Transaction;

pub trait Dao<'a> {
    fn new(tran: &'a Transaction<'a>) -> Self;
    fn tran(&self) -> &Transaction;
}

pub struct BaseDao<'a> {
    tran: &'a Transaction<'a>,
}

impl<'a> Dao<'a> for BaseDao<'a> {
    fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            tran
        }
    }

    fn tran(&self) -> &Transaction {
        self.tran
    }
}

impl<'a> BaseDao<'a> {

}