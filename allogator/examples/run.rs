use std::alloc::Layout;

use gator::Allogator;

const LEN_16_VEC_U64_LAYOUT: Layout =
    unsafe { Layout::from_size_align_unchecked(16 * size_of::<u64>(), align_of::<u64>()) };

static ALLOGATOR: (Allogator, &[usize]) = const {
    let mut allogator = Allogator::new();
    let ptr_1 = allogator.const_allocate(LEN_16_VEC_U64_LAYOUT);
    let ptr_2 = allogator.const_allocate(LEN_16_VEC_U64_LAYOUT);

    (allogator, &[ptr_1, ptr_2])
};

fn main() {
    println!("allogator {:x?}", &ALLOGATOR);
}
