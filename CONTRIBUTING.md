# Contributing to libbluetooth

libbluetooth welcomes contributions from everyone in the form of suggestions, bug reports, pull requests, and feedback. This document gives some guidance if you are thinking of helping us.

Please reach out here in a GitHub issue if we can do anything to help you contribute.

## Submitting bug reports and feature requests

When reporting a bug or asking for help, please include enough details so that the people helping you can reproduce the behavior you are seeing. For some tips on how to approach this, read about how to produce a [Minimal, Complete, and Verifiable example](https://stackoverflow.com/help/mcve).

When making a feature request, please make it clear what problem you intend to solve with the feature, any ideas for how libbluetooth could support solving that problem, any possible alternatives, and any disadvantages.

## Guidelines

* Run `rustfmt` on your code before committing.
* All definitions go into the source file that directly maps to the header the definition is from.
* Definitions are defined in the same order as they are in the original header.
* Imports should be in asciibetical order.

```Rust
use bluetooth::bdaddr_t;
use hci::{
    hci_dev_info, hci_filter, inquiry_info, HCI_FLT_EVENT_BITS, HCI_FLT_TYPE_BITS, HCI_VENDOR_PKT,
};
```

## Newlines and indentation

* Do not use aligned indentation. Indentation should always be block indentation.
* Always use spaces for indentation.
* Remove blank lines.
* Files must end with a trailing newline.

## Constants

* Convert macro constants to Rust constants.
* The type of the constant should depend on where the constant is used.
* If the constant has an unsigned type, but the literal needs to be negative, perform a cast such as `-1i16 as u16`. Use the primitive integer types that correspond to the type of the constant.
* If the constant is initialized to an expression involving a constant of a different type and a cast must be performed, do the cast using the primitive integer types.


## Inline functions and macros

* Inline functions and macros should be translated into Rust functions.
* These functions should always be marked `#[inline]`.
* Until constant functions can be defined in the minimum Rust that libbluetooth supports, if a function needs to be called in a constant, then a macro version of the function should be added.
* Inline functions are allowed to undergo some Rustification because they are not required to match the ABI of the original. E.g. raw pointers can be translated into references.

## Structs

* One field per line.

```C
struct bt_security {
    uint8_t level;
    uint8_t key_size;
};
```

```Rust
STRUCT! {struct bt_security {
    level: u8,
    key_size: u8,
}}
```

## Unions

* The first parameter to `UNION!` is the storage for that union. It must have both the correct size and alignment. You can use the following C++ code to print out the storage for any union type that can be named.

```C++
#include <cstdint>
#include <iostream>

#include <bluetooth/bluetooth.h>

char const * type_for_alignment(uintptr_t align) {
    switch (align) {
    case 1: return "u8";
    case 2: return "u16";
    case 4: return "u32";
    case 8: return "u64";
    default: throw;
    }
}
#define PRINT_UNION(x) std::cout << "[" << type_for_alignment(alignof(x))\
    << "; " << sizeof(x) / alignof(x) << "]" << std::endl;
int main() {
    PRINT_UNION(bdaddr_t);
}
````

* Note that sometimes the storage of a union varies based on whether the target is 32bit or 64bit, in which case `UNION!` allows a second storage to be specified, the first for 32bit and the second for 64bit.

## Anonymous unions and structs

* If the type `foo` contains a single anonymous struct or union, give the anonymous struct or union a name of `foo_s` or `foo_u` respectively, and the field a name of `s` or `u` respectively.
* If the type `foo` contains multiple anonymous structs or unions, append a number, such as `s1: foo_s1` `s2: foo_s2` or `u1: foo_u1` `u2: foo_u2`.
* If the field does have a name, such as `bar`, but still contains an anonymous struct or union then retain the name of `bar` for the field and name the anonymous struct or union after the field, such as `foo_bar`.

## Dealing with duplicates

* Sometimes two headers will define the same thing.
    * If the duplicated thing is a simple typedef or extern function or constant, then duplicate the definition.
    * If the duplicated thing is a struct or union, then choose one header to be the canonical source of truth for that definition and publicly re-export the thing from the other header.

## Conduct

In all libbluetooth-related forums, we follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
