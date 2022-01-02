struct joypad {
    mmu: Rc<RefCell<MMU>>,
}

impl joypad {
    pub fn new(mmu: Rc<RefCell<MMU>>) -> Self {
        Self { mmu: mmu }
    }

    
}
