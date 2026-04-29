/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! AVAudioSession stub.

use crate::objc::{id, ClassExports, HostObject, objc_classes, nil};
use crate::dyld::{ConstantExports, HostConstant};
use crate::frameworks::foundation::ns_string;
use crate::mem::MutPtr;

struct AVAudioSessionHostObject {}
impl HostObject for AVAudioSessionHostObject {}

pub const CONSTANTS: ConstantExports = &[
    ("_AVAudioSessionCategoryAmbient", HostConstant::NSString("AVAudioSessionCategoryAmbient")),
    ("_AVAudioSessionCategorySoloAmbient", HostConstant::NSString("AVAudioSessionCategorySoloAmbient")),
];

pub const CLASSES: ClassExports = objc_classes! {
    (env, this, _cmd);

    @implementation AVAudioSession: NSObject

    + (id)sharedInstance {
        log!("[(AVAudioSession*) sharedInstance]");
        this
    }

    + (bool)setCategory:(id)category error:(id)error {
        log!("[(AVAudioSession+) setCategory:{:?} error:{:?}]", category, error);
        true
    }

    - (bool)setActive:(bool)active error:(id)error {
        log!("[(AVAudioSession*) setActive:{:?} error:{:?}]", active, error);
        true
    }

    - (bool)setCategory:(id)category error:(id)error {
        log!("[(AVAudioSession*) setCategory:{:?} error:{:?}]", category, error);
        true
    }

    - (id)category {
        log!("[(AVAudioSession*) category]");
        nil
    }

    - (bool)setDelegate:(id)delegate {
        log!("[(AVAudioSession*) setDelegate:{:?}]", delegate);
        true
    }

    - (bool)setPreferredIOBufferDuration:(f64)duration error:(id)error {
        log!("[(AVAudioSession*) setPreferredIOBufferDuration:{:?} error:{:?}]", duration, error);
        true
    }

    @end
};
