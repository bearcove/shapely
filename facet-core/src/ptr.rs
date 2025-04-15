//! Opaque pointers
//!
//! Type-erased pointer helpers for working with reflected values

use core::{marker::PhantomData, ptr::NonNull};

/// A type-erased pointer to an uninitialized value
#[derive(Debug, Clone, Copy)]
pub struct PtrUninit<'mem>(*mut u8, PhantomData<&'mem mut ()>);

impl<'mem> PtrUninit<'mem> {
    /// Create a new opaque pointer from a mutable pointer
    ///
    /// This is safe because it's generic over T
    pub fn new<T>(ptr: *mut T) -> Self {
        Self(ptr as *mut u8, PhantomData)
    }

    /// Creates a new opaque pointer from a reference to a [`core::mem::MaybeUninit`]
    ///
    /// The pointer will point to the potentially uninitialized contents
    ///
    /// This is safe because it's generic over T
    pub fn from_maybe_uninit<T>(borrow: &'mem mut core::mem::MaybeUninit<T>) -> Self {
        Self(borrow.as_mut_ptr() as *mut u8, PhantomData)
    }

    /// Assumes the pointer is initialized and returns an `Opaque` pointer
    ///
    /// # Safety
    ///
    /// The pointer must actually be pointing to initialized memory of the correct type.
    pub unsafe fn assume_init(self) -> PtrMut<'mem> {
        let ptr = unsafe { NonNull::new_unchecked(self.0) };
        PtrMut(ptr, PhantomData)
    }

    /// Write a value to this location and convert to an initialized pointer
    ///
    /// # Safety
    ///
    /// The pointer must be properly aligned for T and point to allocated memory
    /// that can be safely written to.
    pub unsafe fn put<T>(self, value: T) -> PtrMut<'mem> {
        unsafe {
            core::ptr::write(self.0 as *mut T, value);
            self.assume_init()
        }
    }

    /// Returns the underlying raw pointer as a byte pointer
    pub fn as_mut_byte_ptr(self) -> *mut u8 {
        self.0
    }

    /// Returns the underlying raw pointer as a const byte pointer
    pub fn as_byte_ptr(self) -> *const u8 {
        self.0
    }

    /// Returns a pointer with the given offset added
    ///
    /// # Safety
    ///
    /// Offset is within the bounds of the allocated memory
    pub unsafe fn field_uninit_at(self, offset: usize) -> PtrUninit<'mem> {
        PtrUninit(unsafe { self.0.byte_add(offset) }, PhantomData)
    }

    /// Returns a pointer with the given offset added, assuming it's initialized
    ///
    /// # Safety
    ///
    /// The pointer plus offset must be:
    /// - Within bounds of the allocated object
    /// - Properly aligned for the type being pointed to
    /// - Point to initialized data of the correct type
    pub unsafe fn field_init_at(self, offset: usize) -> PtrMut<'mem> {
        PtrMut(
            unsafe { NonNull::new_unchecked(self.0.add(offset)) },
            PhantomData,
        )
    }
}

/// A type-erased read-only pointer to an initialized value.
///
/// Cannot be null. May be dangling (for ZSTs)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PtrConst<'mem>(NonNull<u8>, PhantomData<&'mem ()>);

unsafe impl Send for PtrConst<'_> {}
unsafe impl Sync for PtrConst<'_> {}

impl<'mem> PtrConst<'mem> {
    /// Create a new opaque const pointer from a raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be non-null, valid, aligned, and point to initialized memory
    /// of the correct type, and be valid for lifetime `'mem`.
    ///
    /// It's encouraged to take the address of something with `&raw const x`, rather than `&x`
    pub const fn new<T>(ptr: *const T) -> Self {
        unsafe { Self(NonNull::new_unchecked(ptr as *mut u8), PhantomData) }
    }

    /// Gets the underlying raw pointer as a byte pointer
    pub const fn as_byte_ptr(self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Gets the underlying raw pointer as a pointer of type T
    ///
    /// # Safety
    ///
    /// Must be called with the original type T that was used to create this pointer
    pub const unsafe fn as_ptr<T>(self) -> *const T {
        self.0.as_ptr() as *const T
    }

    /// Gets the underlying raw pointer as a const pointer of type T
    ///
    /// # Safety
    ///
    /// `T` must be the _actual_ underlying type. You're downcasting with no guardrails.
    pub const unsafe fn get<'borrow: 'mem, T>(self) -> &'borrow T {
        // TODO: rename to `get`, or something else? it's technically a borrow...
        unsafe { &*(self.0.as_ptr() as *const T) }
    }

    /// Returns a pointer with the given offset added
    ///
    /// # Safety
    ///
    /// Offset must be within the bounds of the allocated memory,
    /// and the resulting pointer must be properly aligned.
    pub const unsafe fn field(self, offset: usize) -> PtrConst<'mem> {
        PtrConst(
            unsafe { NonNull::new_unchecked(self.0.as_ptr().byte_add(offset)) },
            PhantomData,
        )
    }

    /// Exposes [`core::ptr::read`]
    ///
    /// # Safety
    ///
    /// `T` must be the actual underlying type of the pointed-to memory.
    /// The memory must be properly initialized and aligned for type `T`.
    pub const unsafe fn read<T>(self) -> T {
        unsafe { core::ptr::read(self.as_ptr()) }
    }
}

/// A type-erased pointer to an initialized value
#[derive(Clone, Copy)]
pub struct PtrMut<'mem>(NonNull<u8>, PhantomData<&'mem mut ()>);

impl<'mem> PtrMut<'mem> {
    /// Create a new opaque pointer from a raw pointer
    ///
    /// # Safety
    ///
    /// The pointer must be valid, aligned, and point to initialized memory
    /// of the correct type, and be valid for lifetime `'mem`.
    ///
    /// It's encouraged to take the address of something with `&raw mut x`, rather than `&x`
    pub const fn new<T>(ptr: *mut T) -> Self {
        Self(
            unsafe { NonNull::new_unchecked(ptr as *mut u8) },
            PhantomData,
        )
    }

    /// Gets the underlying raw pointer
    pub const fn as_byte_ptr(self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Gets the underlying raw pointer as mutable
    pub const fn as_mut_byte_ptr(self) -> *mut u8 {
        self.0.as_ptr()
    }

    /// Gets the underlying raw pointer as a pointer of type T
    ///
    /// # Safety
    ///
    /// Must be called with the original type T that was used to create this pointer
    pub const unsafe fn as_ptr<T>(self) -> *const T {
        self.0.as_ptr() as *const T
    }

    /// Gets the underlying raw pointer as a mutable pointer of type T
    ///
    /// # Safety
    ///
    /// `T` must be the _actual_ underlying type. You're downcasting with no guardrails.
    pub const unsafe fn as_mut<'borrow: 'mem, T>(self) -> &'borrow mut T {
        unsafe { &mut *(self.0.as_ptr() as *mut T) }
    }

    /// Gets the underlying raw pointer as a const pointer of type T
    ///
    /// # Safety
    ///
    /// `T` must be the _actual_ underlying type. You're downcasting with no guardrails.
    /// You must respect AXM (aliasing xor mutability). Holding onto the borrow while
    /// calling as_mut is UB.
    ///
    /// Basically this is UB land. Careful.
    pub const unsafe fn get<'borrow: 'mem, T>(self) -> &'borrow T {
        unsafe { &*(self.0.as_ptr() as *const T) }
    }

    /// Make a const ptr out of this mut ptr
    pub const fn as_const<'borrow: 'mem>(self) -> PtrConst<'borrow> {
        PtrConst(self.0, PhantomData)
    }

    /// Exposes [`core::ptr::read`]
    ///
    /// # Safety
    ///
    /// `T` must be the actual underlying type of the pointed-to memory.
    /// The memory must be properly initialized and aligned for type `T`.
    pub const unsafe fn read<T>(self) -> T {
        unsafe { core::ptr::read(self.as_mut()) }
    }

    /// Exposes [`core::ptr::drop_in_place`]
    ///
    /// # Safety
    ///
    /// `T` must be the actual underlying type of the pointed-to memory.
    /// The memory must be properly initialized and aligned for type `T`.
    /// After calling this function, the memory should not be accessed again
    /// until it is properly reinitialized.
    pub unsafe fn drop_in_place<T>(self) -> PtrUninit<'mem> {
        unsafe { core::ptr::drop_in_place(self.as_mut::<T>()) }
        PtrUninit(self.0.as_ptr(), PhantomData)
    }

    /// Write a value to this location after dropping the existing value
    ///
    /// # Safety
    ///
    /// - The pointer must be properly aligned for T and point to allocated memory
    ///   that can be safely written to.
    /// - T must be the actual type of the object being pointed to
    /// - The memory must already be initialized to a valid T value
    pub unsafe fn replace<T>(self, value: T) -> Self {
        unsafe { self.drop_in_place::<T>().put(value) }
    }

    /// Copies data from the source pointer to this location
    ///
    /// # Safety
    ///
    /// - The destination pointer must be properly aligned and point to allocated memory
    ///   that can be safely written to.
    /// - The source pointer must point to properly initialized data.
    /// - Both pointers must refer to objects of the same type and size.
    pub unsafe fn write(self, source: PtrConst<'_>) -> Self {
        unsafe {
            let size = core::mem::size_of_val(&*source.as_byte_ptr());
            core::ptr::copy_nonoverlapping(source.as_byte_ptr(), self.0.as_ptr(), size);
            self
        }
    }
}
