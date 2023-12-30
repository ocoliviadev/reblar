pub mod traits;
pub mod media;

#[macro_export]
macro_rules! return_err {
    ( $res_returning_op:expr ) => {
        {
            let res = $res_returning_op;
            let is_err = res.is_err();
            if is_err {
                return Err(res.err().unwrap().into())
            }
            res.unwrap()
        }
    }
}
