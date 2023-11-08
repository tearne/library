trait ObjSafe {
    fn get_string(&self) -> String;
}

trait Not {
    fn new(n: u64) -> Self;
    fn add(&mut self, other: Self);
    fn duplicate(&self) -> Self;
}

trait Everything {
    fn get_string(&self) -> String;
    fn new(n: u64) -> Self;
    fn add(&mut self, other: Self);
    fn duplicate(&self) -> Self;
}

struct A {}
impl Not for A {
    fn new(_n: u64) -> Self {
        todo!()
    }
    fn add(&mut self, _other: Self) {
        todo!()
    }
    fn duplicate(&self) -> Self {
        todo!()
    }
}
impl ObjSafe for A {
    fn get_string(&self) -> String {
        todo!()
    }
}

#[allow(clippy::vec_init_then_push)]
pub fn main() {
    //Compiles
    let mut items: Vec<Box<dyn ObjSafe>> = Vec::new();
    items.push(Box::new(A{}));

    //Doesn't
    // let _items: Vec<Box<dyn Not>> = Vec::new();
}
