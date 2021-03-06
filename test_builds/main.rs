#![feature(prelude_import)]
#![feature(arbitrary_enum_discriminant)]
#![feature(box_syntax, test, fmt_internals)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate packattack_derive;
use packattack::FromByte;
use packattack::FromBytes;
mod connect {
    pub type KeepAlive = u16;
    pub type Protocol = String;
    pub type ProtocolLevel = u8;
    use super::qos::QoS;
    use packattack::FromByte;
    use packattack::FromBytes;
    use std::error;
    use std::fmt;
    pub struct ConnectFlags {
        #[start_byte]
        UserNameFlag: bool,
        PasswordFlag: bool,
        WillRetain: bool,
        WillQoS: QoS,
        WillFlag: bool,
        #[end_byte]
        CleanStart: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ConnectFlags {
        #[inline]
        fn clone(&self) -> ConnectFlags {
            {
                let _: ::core::clone::AssertParamIsClone<bool>;
                let _: ::core::clone::AssertParamIsClone<bool>;
                let _: ::core::clone::AssertParamIsClone<bool>;
                let _: ::core::clone::AssertParamIsClone<QoS>;
                let _: ::core::clone::AssertParamIsClone<bool>;
                let _: ::core::clone::AssertParamIsClone<bool>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for ConnectFlags {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ConnectFlags {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ConnectFlags {
                    UserNameFlag: ref __self_0_0,
                    PasswordFlag: ref __self_0_1,
                    WillRetain: ref __self_0_2,
                    WillQoS: ref __self_0_3,
                    WillFlag: ref __self_0_4,
                    CleanStart: ref __self_0_5,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ConnectFlags");
                    let _ = debug_trait_builder.field("UserNameFlag", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("PasswordFlag", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("WillRetain", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("WillQoS", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("WillFlag", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("CleanStart", &&(*__self_0_5));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ConnectFlags {
        #[inline]
        fn eq(&self, other: &ConnectFlags) -> bool {
            match *other {
                ConnectFlags {
                    UserNameFlag: ref __self_1_0,
                    PasswordFlag: ref __self_1_1,
                    WillRetain: ref __self_1_2,
                    WillQoS: ref __self_1_3,
                    WillFlag: ref __self_1_4,
                    CleanStart: ref __self_1_5,
                } => match *self {
                    ConnectFlags {
                        UserNameFlag: ref __self_0_0,
                        PasswordFlag: ref __self_0_1,
                        WillRetain: ref __self_0_2,
                        WillQoS: ref __self_0_3,
                        WillFlag: ref __self_0_4,
                        CleanStart: ref __self_0_5,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ConnectFlags) -> bool {
            match *other {
                ConnectFlags {
                    UserNameFlag: ref __self_1_0,
                    PasswordFlag: ref __self_1_1,
                    WillRetain: ref __self_1_2,
                    WillQoS: ref __self_1_3,
                    WillFlag: ref __self_1_4,
                    CleanStart: ref __self_1_5,
                } => match *self {
                    ConnectFlags {
                        UserNameFlag: ref __self_0_0,
                        PasswordFlag: ref __self_0_1,
                        WillRetain: ref __self_0_2,
                        WillQoS: ref __self_0_3,
                        WillFlag: ref __self_0_4,
                        CleanStart: ref __self_0_5,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                    }
                },
            }
        }
    }
    impl FromBytes for ConnectFlags {
        #[allow(trivial_numeric_casts)]
        fn read_from_bytes(bytes: &[u8], count: &mut usize) -> ConnectFlags {
            ConnectFlags {
                UserNameFlag: <bool>::read_from_byte(&bytes[*count], <bool>::bitmask >> (0)),
                PasswordFlag: <bool>::read_from_byte(
                    &bytes[*count],
                    <bool>::bitmask >> (0 + <bool>::size_in_bits),
                ),
                WillRetain: <bool>::read_from_byte(
                    &bytes[*count],
                    <bool>::bitmask >> (0 + <bool>::size_in_bits + <bool>::size_in_bits),
                ),
                WillQoS: <QoS>::read_from_byte(
                    &bytes[*count],
                    <QoS>::bitmask
                        >> (0 + <bool>::size_in_bits + <bool>::size_in_bits + <bool>::size_in_bits),
                ),
                WillFlag: <bool>::read_from_byte(
                    &bytes[*count],
                    <bool>::bitmask
                        >> (0
                            + <bool>::size_in_bits
                            + <bool>::size_in_bits
                            + <bool>::size_in_bits
                            + <QoS>::size_in_bits),
                ),
                CleanStart: {
                    *count += 1;
                    <bool>::read_from_byte(
                        &bytes[*count - 1],
                        <bool>::bitmask
                            >> (0
                                + <bool>::size_in_bits
                                + <bool>::size_in_bits
                                + <bool>::size_in_bits
                                + <QoS>::size_in_bits
                                + <bool>::size_in_bits),
                    )
                },
            }
        }
    }
    pub struct MalformedConnectFlagsError;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MalformedConnectFlagsError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MalformedConnectFlagsError => {
                    let mut debug_trait_builder = f.debug_tuple("MalformedConnectFlagsError");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MalformedConnectFlagsError {
        #[inline]
        fn clone(&self) -> MalformedConnectFlagsError {
            match *self {
                MalformedConnectFlagsError => MalformedConnectFlagsError,
            }
        }
    }
    impl fmt::Display for MalformedConnectFlagsError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["The packet was malformed! Reserved Flag in CONNECT Packet was set to 1."],
                &match () {
                    () => [],
                },
            ))
        }
    }
    impl error::Error for MalformedConnectFlagsError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            None
        }
    }
}
mod qos {
    use packattack::FromByte;
    #[repr(u8)]
    #[size_in_bits = 3]
    pub enum QoS {
        AtMostOnce = 0,
        AtLeastOnce = 1,
        ExactlyOnce = 2,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for QoS {
        #[inline]
        fn clone(&self) -> QoS {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for QoS {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for QoS {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&QoS::AtMostOnce,) => {
                    let mut debug_trait_builder = f.debug_tuple("AtMostOnce");
                    debug_trait_builder.finish()
                }
                (&QoS::AtLeastOnce,) => {
                    let mut debug_trait_builder = f.debug_tuple("AtLeastOnce");
                    debug_trait_builder.finish()
                }
                (&QoS::ExactlyOnce,) => {
                    let mut debug_trait_builder = f.debug_tuple("ExactlyOnce");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for QoS {
        #[inline]
        fn eq(&self, other: &QoS) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as u8;
                let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as u8;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    impl FromByte for QoS {
        #[inline(always)]
        fn read_from_byte(byte: &u8, mask: u8) -> QoS {
            let value: u8 = byte & mask;
            match value {
                0 => QoS::AtMostOnce,
                1 => QoS::AtLeastOnce,
                2 => QoS::ExactlyOnce,
                _ => ::std::rt::begin_panic("uh oh no match", &("src\\qos.rs", 3u32, 41u32)),
            }
        }
        const size_in_bits: u8 = 3u8;
        const bitmask: u8 = 224u8;
    }
}
#[allow(dead_code)]
#[repr(u8)]
enum Packet {
    CONNECT(
        connect::Protocol,
        connect::ProtocolLevel,
        connect::ConnectFlags,
        connect::KeepAlive,
    ) = 0,
    CONNACK = 1,
    PUBLISH = 2,
    PUBACK = 3,
    PUBREC = 4,
    PUBREL = 5,
    PUBCOMP = 6,
    SUBSCRIBE = 7,
    SUBACK = 8,
    UNSUBSCRIBE = 9,
    UNSUBACK = 10,
    PINGREQ = 11,
    PINGRESP = 12,
    DISCONNECT = 13,
    AUTH = 14,
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code)]
impl ::core::clone::Clone for Packet {
    #[inline]
    fn clone(&self) -> Packet {
        match (&*self,) {
            (&Packet::CONNECT(ref __self_0, ref __self_1, ref __self_2, ref __self_3),) => {
                Packet::CONNECT(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                    ::core::clone::Clone::clone(&(*__self_3)),
                )
            }
            (&Packet::CONNACK,) => Packet::CONNACK,
            (&Packet::PUBLISH,) => Packet::PUBLISH,
            (&Packet::PUBACK,) => Packet::PUBACK,
            (&Packet::PUBREC,) => Packet::PUBREC,
            (&Packet::PUBREL,) => Packet::PUBREL,
            (&Packet::PUBCOMP,) => Packet::PUBCOMP,
            (&Packet::SUBSCRIBE,) => Packet::SUBSCRIBE,
            (&Packet::SUBACK,) => Packet::SUBACK,
            (&Packet::UNSUBSCRIBE,) => Packet::UNSUBSCRIBE,
            (&Packet::UNSUBACK,) => Packet::UNSUBACK,
            (&Packet::PINGREQ,) => Packet::PINGREQ,
            (&Packet::PINGRESP,) => Packet::PINGRESP,
            (&Packet::DISCONNECT,) => Packet::DISCONNECT,
            (&Packet::AUTH,) => Packet::AUTH,
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code)]
impl ::core::fmt::Debug for Packet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Packet::CONNECT(ref __self_0, ref __self_1, ref __self_2, ref __self_3),) => {
                let mut debug_trait_builder = f.debug_tuple("CONNECT");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                let _ = debug_trait_builder.field(&&(*__self_2));
                let _ = debug_trait_builder.field(&&(*__self_3));
                debug_trait_builder.finish()
            }
            (&Packet::CONNACK,) => {
                let mut debug_trait_builder = f.debug_tuple("CONNACK");
                debug_trait_builder.finish()
            }
            (&Packet::PUBLISH,) => {
                let mut debug_trait_builder = f.debug_tuple("PUBLISH");
                debug_trait_builder.finish()
            }
            (&Packet::PUBACK,) => {
                let mut debug_trait_builder = f.debug_tuple("PUBACK");
                debug_trait_builder.finish()
            }
            (&Packet::PUBREC,) => {
                let mut debug_trait_builder = f.debug_tuple("PUBREC");
                debug_trait_builder.finish()
            }
            (&Packet::PUBREL,) => {
                let mut debug_trait_builder = f.debug_tuple("PUBREL");
                debug_trait_builder.finish()
            }
            (&Packet::PUBCOMP,) => {
                let mut debug_trait_builder = f.debug_tuple("PUBCOMP");
                debug_trait_builder.finish()
            }
            (&Packet::SUBSCRIBE,) => {
                let mut debug_trait_builder = f.debug_tuple("SUBSCRIBE");
                debug_trait_builder.finish()
            }
            (&Packet::SUBACK,) => {
                let mut debug_trait_builder = f.debug_tuple("SUBACK");
                debug_trait_builder.finish()
            }
            (&Packet::UNSUBSCRIBE,) => {
                let mut debug_trait_builder = f.debug_tuple("UNSUBSCRIBE");
                debug_trait_builder.finish()
            }
            (&Packet::UNSUBACK,) => {
                let mut debug_trait_builder = f.debug_tuple("UNSUBACK");
                debug_trait_builder.finish()
            }
            (&Packet::PINGREQ,) => {
                let mut debug_trait_builder = f.debug_tuple("PINGREQ");
                debug_trait_builder.finish()
            }
            (&Packet::PINGRESP,) => {
                let mut debug_trait_builder = f.debug_tuple("PINGRESP");
                debug_trait_builder.finish()
            }
            (&Packet::DISCONNECT,) => {
                let mut debug_trait_builder = f.debug_tuple("DISCONNECT");
                debug_trait_builder.finish()
            }
            (&Packet::AUTH,) => {
                let mut debug_trait_builder = f.debug_tuple("AUTH");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code)]
impl ::core::cmp::PartialEq for Packet {
    #[inline]
    fn eq(&self, other: &Packet) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as u8;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as u8;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &Packet::CONNECT(ref __self_0, ref __self_1, ref __self_2, ref __self_3),
                        &Packet::CONNECT(
                            ref __arg_1_0,
                            ref __arg_1_1,
                            ref __arg_1_2,
                            ref __arg_1_3,
                        ),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                            && (*__self_3) == (*__arg_1_3)
                    }
                    _ => true,
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Packet) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as u8;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as u8;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &Packet::CONNECT(ref __self_0, ref __self_1, ref __self_2, ref __self_3),
                        &Packet::CONNECT(
                            ref __arg_1_0,
                            ref __arg_1_1,
                            ref __arg_1_2,
                            ref __arg_1_3,
                        ),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                            || (*__self_3) != (*__arg_1_3)
                    }
                    _ => false,
                }
            } else {
                true
            }
        }
    }
}
impl FromBytes for Packet {
    fn read_from_bytes(bytes: &[u8], count: &mut usize) -> Packet {
        match &bytes[0] {
            0 => Packet::CONNECT(
                <connect::Protocol>::read_from_bytes(&bytes, count),
                <connect::ProtocolLevel>::read_from_bytes(&bytes, count),
                <connect::ConnectFlags>::read_from_bytes(&bytes, count),
                <connect::KeepAlive>::read_from_bytes(&bytes, count),
            ),
            1 => Packet::CONNACK,
            2 => Packet::PUBLISH,
            3 => Packet::PUBACK,
            4 => Packet::PUBREC,
            5 => Packet::PUBREL,
            6 => Packet::PUBCOMP,
            7 => Packet::SUBSCRIBE,
            8 => Packet::SUBACK,
            9 => Packet::UNSUBSCRIBE,
            10 => Packet::UNSUBACK,
            11 => Packet::PINGREQ,
            12 => Packet::PINGRESP,
            13 => Packet::DISCONNECT,
            14 => Packet::AUTH,
            _ => ::std::rt::begin_panic("uh oh no match", &("src\\lib.rs", 15u32, 35u32)),
        }
    }
}
