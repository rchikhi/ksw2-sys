#![allow(non_camel_case_types)]
/* automatically generated by rust-bindgen 0.60.1 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __uint32_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ksw_extz_t {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub max_q: ::std::os::raw::c_int,
    pub max_t: ::std::os::raw::c_int,
    pub mqe: ::std::os::raw::c_int,
    pub mqe_t: ::std::os::raw::c_int,
    pub mte: ::std::os::raw::c_int,
    pub mte_q: ::std::os::raw::c_int,
    pub score: ::std::os::raw::c_int,
    pub m_cigar: ::std::os::raw::c_int,
    pub n_cigar: ::std::os::raw::c_int,
    pub reach_end: ::std::os::raw::c_int,
    pub cigar: *mut u32,
}
#[test]
fn bindgen_test_layout_ksw_extz_t() {
    assert_eq!(
        ::std::mem::size_of::<ksw_extz_t>(),
        56usize,
        concat!("Size of: ", stringify!(ksw_extz_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ksw_extz_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ksw_extz_t))
    );
    fn test_field_max_q() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ksw_extz_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).max_q) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(ksw_extz_t),
                "::",
                stringify!(max_q)
            )
        );
    }
    test_field_max_q();
    fn test_field_max_t() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ksw_extz_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).max_t) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ksw_extz_t),
                "::",
                stringify!(max_t)
            )
        );
    }
    test_field_max_t();
    fn test_field_mqe() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ksw_extz_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mqe) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(ksw_extz_t),
                "::",
                stringify!(mqe)
            )
        );
    }
    test_field_mqe();
    fn test_field_mqe_t() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ksw_extz_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mqe_t) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(ksw_extz_t),
                "::",
                stringify!(mqe_t)
            )
        );
    }
    test_field_mqe_t();
    fn test_field_mte() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ksw_extz_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mte) as usize - ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(ksw_extz_t),
                "::",
                stringify!(mte)
            )
        );
    }
    test_field_mte();
    fn test_field_mte_q() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ksw_extz_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mte_q) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(ksw_extz_t),
                "::",
                stringify!(mte_q)
            )
        );
    }
    test_field_mte_q();
    fn test_field_score() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ksw_extz_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).score) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(ksw_extz_t),
                "::",
                stringify!(score)
            )
        );
    }
    test_field_score();
    fn test_field_m_cigar() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ksw_extz_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).m_cigar) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(ksw_extz_t),
                "::",
                stringify!(m_cigar)
            )
        );
    }
    test_field_m_cigar();
    fn test_field_n_cigar() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ksw_extz_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).n_cigar) as usize - ptr as usize
            },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(ksw_extz_t),
                "::",
                stringify!(n_cigar)
            )
        );
    }
    test_field_n_cigar();
    fn test_field_reach_end() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ksw_extz_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).reach_end) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(ksw_extz_t),
                "::",
                stringify!(reach_end)
            )
        );
    }
    test_field_reach_end();
    fn test_field_cigar() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ksw_extz_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cigar) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(ksw_extz_t),
                "::",
                stringify!(cigar)
            )
        );
    }
    test_field_cigar();
}
impl ksw_extz_t {
    #[inline]
    pub fn max(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 31u8) as u32) }
    }
    #[inline]
    pub fn set_max(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 31u8, val as u64)
        }
    }
    #[inline]
    pub fn zdropped(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_zdropped(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(max: u32, zdropped: u32) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 31u8, {
            let max: u32 = unsafe { ::std::mem::transmute(max) };
            max as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let zdropped: u32 = unsafe { ::std::mem::transmute(zdropped) };
            zdropped as u64
        });
        __bindgen_bitfield_unit
    }
}
extern "C" {
    #[doc = " NW-like extension"]
    #[doc = ""]
    #[doc = " @param km        memory pool, when used with kalloc"]
    #[doc = " @param qlen      query length"]
    #[doc = " @param query     query sequence with 0 <= query[i] < m"]
    #[doc = " @param tlen      target length"]
    #[doc = " @param target    target sequence with 0 <= target[i] < m"]
    #[doc = " @param m         number of residue types"]
    #[doc = " @param mat       m*m scoring mattrix in one-dimension array"]
    #[doc = " @param gapo      gap open penalty; a gap of length l cost \"-(gapo+l*gape)\""]
    #[doc = " @param gape      gap extension penalty"]
    #[doc = " @param w         band width (<0 to disable)"]
    #[doc = " @param zdrop     off-diagonal drop-off to stop extension (positive; <0 to disable)"]
    #[doc = " @param flag      flag (see KSW_EZ_* macros)"]
    #[doc = " @param ez        (out) scores and cigar"]
    pub fn ksw_extz(
        km: *mut ::std::os::raw::c_void,
        qlen: ::std::os::raw::c_int,
        query: *const u8,
        tlen: ::std::os::raw::c_int,
        target: *const u8,
        m: i8,
        mat: *const i8,
        q: i8,
        e: i8,
        w: ::std::os::raw::c_int,
        zdrop: ::std::os::raw::c_int,
        flag: ::std::os::raw::c_int,
        ez: *mut ksw_extz_t,
    );
}
extern "C" {
    pub fn ksw_extz2_sse(
        km: *mut ::std::os::raw::c_void,
        qlen: ::std::os::raw::c_int,
        query: *const u8,
        tlen: ::std::os::raw::c_int,
        target: *const u8,
        m: i8,
        mat: *const i8,
        q: i8,
        e: i8,
        w: ::std::os::raw::c_int,
        zdrop: ::std::os::raw::c_int,
        end_bonus: ::std::os::raw::c_int,
        flag: ::std::os::raw::c_int,
        ez: *mut ksw_extz_t,
    );
}
extern "C" {
    pub fn ksw_extd(
        km: *mut ::std::os::raw::c_void,
        qlen: ::std::os::raw::c_int,
        query: *const u8,
        tlen: ::std::os::raw::c_int,
        target: *const u8,
        m: i8,
        mat: *const i8,
        gapo: i8,
        gape: i8,
        gapo2: i8,
        gape2: i8,
        w: ::std::os::raw::c_int,
        zdrop: ::std::os::raw::c_int,
        flag: ::std::os::raw::c_int,
        ez: *mut ksw_extz_t,
    );
}
extern "C" {
    pub fn ksw_extd2_sse(
        km: *mut ::std::os::raw::c_void,
        qlen: ::std::os::raw::c_int,
        query: *const u8,
        tlen: ::std::os::raw::c_int,
        target: *const u8,
        m: i8,
        mat: *const i8,
        gapo: i8,
        gape: i8,
        gapo2: i8,
        gape2: i8,
        w: ::std::os::raw::c_int,
        zdrop: ::std::os::raw::c_int,
        end_bonus: ::std::os::raw::c_int,
        flag: ::std::os::raw::c_int,
        ez: *mut ksw_extz_t,
    );
}
extern "C" {
    pub fn ksw_exts2_sse(
        km: *mut ::std::os::raw::c_void,
        qlen: ::std::os::raw::c_int,
        query: *const u8,
        tlen: ::std::os::raw::c_int,
        target: *const u8,
        m: i8,
        mat: *const i8,
        gapo: i8,
        gape: i8,
        gapo2: i8,
        noncan: i8,
        zdrop: ::std::os::raw::c_int,
        junc_bonus: i8,
        flag: ::std::os::raw::c_int,
        junc: *const u8,
        ez: *mut ksw_extz_t,
    );
}
extern "C" {
    pub fn ksw_extf2_sse(
        km: *mut ::std::os::raw::c_void,
        qlen: ::std::os::raw::c_int,
        query: *const u8,
        tlen: ::std::os::raw::c_int,
        target: *const u8,
        mch: i8,
        mis: i8,
        e: i8,
        w: ::std::os::raw::c_int,
        xdrop: ::std::os::raw::c_int,
        ez: *mut ksw_extz_t,
    );
}
extern "C" {
    #[doc = " Global alignment"]
    #[doc = ""]
    #[doc = " (first 10 parameters identical to ksw_extz_sse())"]
    #[doc = " @param m_cigar   (modified) max CIGAR length; feed 0 if cigar==0"]
    #[doc = " @param n_cigar   (out) number of CIGAR elements"]
    #[doc = " @param cigar     (out) BAM-encoded CIGAR; caller need to deallocate with kfree(km, )"]
    #[doc = ""]
    #[doc = " @return          score of the alignment"]
    pub fn ksw_gg(
        km: *mut ::std::os::raw::c_void,
        qlen: ::std::os::raw::c_int,
        query: *const u8,
        tlen: ::std::os::raw::c_int,
        target: *const u8,
        m: i8,
        mat: *const i8,
        gapo: i8,
        gape: i8,
        w: ::std::os::raw::c_int,
        m_cigar_: *mut ::std::os::raw::c_int,
        n_cigar_: *mut ::std::os::raw::c_int,
        cigar_: *mut *mut u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ksw_gg2(
        km: *mut ::std::os::raw::c_void,
        qlen: ::std::os::raw::c_int,
        query: *const u8,
        tlen: ::std::os::raw::c_int,
        target: *const u8,
        m: i8,
        mat: *const i8,
        gapo: i8,
        gape: i8,
        w: ::std::os::raw::c_int,
        m_cigar_: *mut ::std::os::raw::c_int,
        n_cigar_: *mut ::std::os::raw::c_int,
        cigar_: *mut *mut u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ksw_gg2_sse(
        km: *mut ::std::os::raw::c_void,
        qlen: ::std::os::raw::c_int,
        query: *const u8,
        tlen: ::std::os::raw::c_int,
        target: *const u8,
        m: i8,
        mat: *const i8,
        gapo: i8,
        gape: i8,
        w: ::std::os::raw::c_int,
        m_cigar_: *mut ::std::os::raw::c_int,
        n_cigar_: *mut ::std::os::raw::c_int,
        cigar_: *mut *mut u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ksw_ll_qinit(
        km: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
        qlen: ::std::os::raw::c_int,
        query: *const u8,
        m: ::std::os::raw::c_int,
        mat: *const i8,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn ksw_ll_i16(
        q: *mut ::std::os::raw::c_void,
        tlen: ::std::os::raw::c_int,
        target: *const u8,
        gapo: ::std::os::raw::c_int,
        gape: ::std::os::raw::c_int,
        qe: *mut ::std::os::raw::c_int,
        te: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}