pub mod state{

    use tree::tree::Tree;

    #[derive(Debug, Clone)]
    pub struct State{
        pub pos: i32,
        pub tree: Vec<Tree>
    }

    impl State{
        pub fn set(&mut self, other: State){
            *self = other;
        }
    }
}