#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use gatt::*;
#[repr(transparent)]
pub struct CharacteristicA(Characteristic);
#[repr(transparent)]
pub struct DescriptorA(Descriptor);
struct Attribute {
    /// The type of the attribute as a UUID16, EG "Primary Service" or "Anaerobic Heart Rate Lower Limit"
    pub att_type: usize,
    /// Unique server-side identifer for attribute
    pub handle: usize,
    /// Attribute values can be any fixed length or variable length octet array, which if too large
    /// can be sent across multiple PDUs
    pub value: &'static [u8],
}
struct Descriptor {
    attributes: &'static [Attribute],
}
struct Characteristic {
    attributes: &'static [Attribute],
    descriptors: &'static [Descriptor],
}
struct Service {
    attributes: &'static [Attribute],
    characteristics: &'static [Characteristic],
}
trait ServiceTrait {}
#[repr(transparent)]
pub struct ServiceA(Service);
#[repr(transparent)]
pub struct ServiceB(Service);
#[repr(transparent)]
pub struct AttributeA(Attribute);
#[repr(transparent)]
pub struct AttributeB(Attribute);
#[repr(transparent)]
pub struct AttributeC(Attribute);
#[repr(transparent)]
pub struct AttributeD(Attribute);
impl ServiceA {
    fn kek(&mut self) {}
}
mod gatt_server {
    use super::*;
    static DATA_STORE: [u8; 10usize] = [0; 10usize];
    static ATTRIBUTES: [Attribute; 5usize] = [
        Attribute {
            att_type: 0,
            handle: 0,
            value: unsafe {
                core::mem::transmute::<&'static u8, &'static [u8; 3usize]>(&DATA_STORE[0usize])
            },
        },
        Attribute {
            att_type: 0,
            handle: 0,
            value: unsafe {
                core::mem::transmute::<&'static u8, &'static [u8; 3usize]>(&DATA_STORE[3usize])
            },
        },
        Attribute {
            att_type: 0,
            handle: 0,
            value: unsafe {
                core::mem::transmute::<&'static u8, &'static [u8; 3usize]>(&DATA_STORE[6usize])
            },
        },
        Attribute {
            att_type: 0,
            handle: 0,
            value: unsafe {
                core::mem::transmute::<&'static u8, &'static [u8; 0usize]>(&DATA_STORE[9usize])
            },
        },
        Attribute {
            att_type: 0,
            handle: 0,
            value: unsafe {
                core::mem::transmute::<&'static u8, &'static [u8; 1usize]>(&DATA_STORE[9usize])
            },
        },
    ];
    static SERVICES: [Service; 2usize] = [
        Service {
            attributes: &[],
            characteristics: &[],
        },
        Service {
            attributes: &[],
            characteristics: &[],
        },
    ];
    static CHARACTERISTICS: [Characteristic; 1usize] = [Characteristic {
        attributes: &[],
        descriptors: &[],
    }];
    static DESCRIPTORS: [Descriptor; 1usize] = [Descriptor { attributes: &[] }];
    static mut GAT_SERVER_TAKEN: bool = false;
    pub struct GattServer {}
    impl GattServer {
        pub fn take() -> Option<Self> {
            if unsafe { GAT_SERVER_TAKEN } {
                None
            } else {
                Some(GattServer {})
            }
        }
        pub fn service_a(&mut self) -> ServiceAHandle {
            ServiceAHandle {
                pd: core::marker::PhantomData {},
            }
        }
        pub fn service_b(&mut self) -> ServiceBHandle {
            ServiceBHandle {
                pd: core::marker::PhantomData {},
            }
        }
    }
    pub struct ServiceAHandle<'a> {
        pd: core::marker::PhantomData<&'a mut ()>,
    }
    impl core::ops::Deref for ServiceAHandle<'_> {
        type Target = ServiceA;
        fn deref(&self) -> &Self::Target {
            unsafe { core::mem::transmute(&SERVICES[0usize]) }
        }
    }
    impl core::ops::DerefMut for ServiceAHandle<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                &mut *(core::mem::transmute::<&Service, &ServiceA>(&SERVICES[0usize])
                    as *const ServiceA as *mut ServiceA)
            }
        }
    }
    impl ServiceAHandle<'_> {
        pub fn characteristic_a(&mut self) -> CharacteristicAHandle {
            CharacteristicAHandle {
                pd: core::marker::PhantomData {},
            }
        }
    }
    pub struct ServiceBHandle<'a> {
        pd: core::marker::PhantomData<&'a mut ()>,
    }
    impl core::ops::Deref for ServiceBHandle<'_> {
        type Target = ServiceB;
        fn deref(&self) -> &Self::Target {
            unsafe { core::mem::transmute(&SERVICES[1usize]) }
        }
    }
    impl core::ops::DerefMut for ServiceBHandle<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                &mut *(core::mem::transmute::<&Service, &ServiceB>(&SERVICES[0usize])
                    as *const ServiceB as *mut ServiceB)
            }
        }
    }
    impl ServiceBHandle<'_> {}
    pub struct CharacteristicAHandle<'a> {
        pd: core::marker::PhantomData<&'a mut ()>,
    }
    impl core::ops::Deref for CharacteristicAHandle<'_> {
        type Target = CharacteristicA;
        fn deref(&self) -> &Self::Target {
            unsafe { core::mem::transmute(&CHARACTERISTICS[0usize]) }
        }
    }
    impl core::ops::DerefMut for CharacteristicAHandle<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                &mut *(core::mem::transmute::<&Characteristic, &CharacteristicA>(
                    &CHARACTERISTICS[0usize],
                ) as *const CharacteristicA as *mut CharacteristicA)
            }
        }
    }
    impl CharacteristicAHandle<'_> {
        pub fn descriptor_a(&mut self) -> DescriptorAHandle {
            DescriptorAHandle {
                pd: core::marker::PhantomData {},
            }
        }
    }
    pub struct DescriptorAHandle<'a> {
        pd: core::marker::PhantomData<&'a mut ()>,
    }
    impl core::ops::Deref for DescriptorAHandle<'_> {
        type Target = DescriptorA;
        fn deref(&self) -> &Self::Target {
            unsafe { core::mem::transmute(&DESCRIPTORS[0usize]) }
        }
    }
    impl core::ops::DerefMut for DescriptorAHandle<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                &mut *(core::mem::transmute::<&Descriptor, &DescriptorA>(&DESCRIPTORS[0usize])
                    as *const DescriptorA as *mut DescriptorA)
            }
        }
    }
    impl DescriptorAHandle<'_> {
        pub fn attribute_a(&mut self) -> AttributeAHandle {
            AttributeAHandle {
                inner: unsafe { &mut *(&ATTRIBUTES[0usize] as *const Attribute as *mut Attribute) },
            }
        }
        pub fn b(&mut self) -> AttributeBHandle {
            AttributeBHandle {
                inner: unsafe { &mut *(&ATTRIBUTES[0usize] as *const Attribute as *mut Attribute) },
            }
        }
        pub fn c(&mut self) -> AttributeCHandle {
            AttributeCHandle {
                inner: unsafe { &mut *(&ATTRIBUTES[0usize] as *const Attribute as *mut Attribute) },
            }
        }
    }
    pub struct AttributeAHandle<'a> {
        inner: &'a mut Attribute,
    }
    impl core::ops::Deref for AttributeAHandle<'_> {
        type Target = AttributeA;
        fn deref(&self) -> &Self::Target {
            unsafe { core::mem::transmute(&self.inner) }
        }
    }
    impl core::ops::DerefMut for AttributeAHandle<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                &mut *(core::mem::transmute::<&Attribute, &AttributeA>(&self.inner)
                    as *const AttributeA as *mut AttributeA)
            }
        }
    }
    impl AttributeAHandle<'_> {
        pub fn get(&self) {
            &self.inner.value
        }
    }
    pub struct AttributeBHandle<'a> {
        inner: &'a mut Attribute,
    }
    impl core::ops::Deref for AttributeBHandle<'_> {
        type Target = AttributeB;
        fn deref(&self) -> &Self::Target {
            unsafe { core::mem::transmute(&self.inner) }
        }
    }
    impl core::ops::DerefMut for AttributeBHandle<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                &mut *(core::mem::transmute::<&Attribute, &AttributeB>(&self.inner)
                    as *const AttributeB as *mut AttributeB)
            }
        }
    }
    impl AttributeBHandle<'_> {
        pub fn get(&self) {
            &self.inner.value
        }
    }
    pub struct AttributeCHandle<'a> {
        inner: &'a mut Attribute,
    }
    impl core::ops::Deref for AttributeCHandle<'_> {
        type Target = AttributeC;
        fn deref(&self) -> &Self::Target {
            unsafe { core::mem::transmute(&self.inner) }
        }
    }
    impl core::ops::DerefMut for AttributeCHandle<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                &mut *(core::mem::transmute::<&Attribute, &AttributeC>(&self.inner)
                    as *const AttributeC as *mut AttributeC)
            }
        }
    }
    impl AttributeCHandle<'_> {
        pub fn get(&self) {
            &self.inner.value
        }
    }
    pub struct AttributeDHandle<'a> {
        inner: &'a mut Attribute,
    }
    impl core::ops::Deref for AttributeDHandle<'_> {
        type Target = AttributeD;
        fn deref(&self) -> &Self::Target {
            unsafe { core::mem::transmute(&self.inner) }
        }
    }
    impl core::ops::DerefMut for AttributeDHandle<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                &mut *(core::mem::transmute::<&Attribute, &AttributeD>(&self.inner)
                    as *const AttributeD as *mut AttributeD)
            }
        }
    }
    impl AttributeDHandle<'_> {
        pub fn get(&self) {
            &self.inner.value
        }
    }
}
fn main() {
    let mut server = gatt_server::GattServer::take().unwrap();
    server.service_a().kek();
    let val = server
        .service_a()
        .characteristic_a()
        .descriptor_a()
        .attribute_a()
        .get();
    server
        .service_a()
        .characteristic_a()
        .descriptor_a()
        .b()
        .set(val);
}
