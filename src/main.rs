

use core::marker::PhantomData;

#[derive(Debug)]
struct RefMut<'a,T>{
    _p:PhantomData<&'a mut T>,
    val:u16
}

struct Base{
    a:usize
}
impl Drop for Base{
    fn drop(&mut self){
        dbg!("drop");
    }
}
impl Base{
    fn new()->Base{
        Base{a:0}
    }
    
    fn borrow<'a,T>(&'a mut self,arr:&'a mut [T])->impl Iterator<Item=RefMut<'a,T>>{
        assert!(arr.len()<u16::MAX.into());
        arr.iter_mut().map(|x|RefMut{_p:PhantomData,val:0})
    }
}



fn main(){
    
    let mut arr=[0usize,5,2,3];
    
    let mut base=Base::new();
    
    let mut stuff=base.borrow(&mut arr);
    
    
    
    dbg!(stuff.next());
}

