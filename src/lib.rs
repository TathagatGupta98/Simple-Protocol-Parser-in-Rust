#[derive(Eq,PartialEq,Debug)]
pub enum Command{
    Publish(String),
    Retrieve,
}

#[derive(Eq,PartialEq,Debug)]
pub enum error{

}