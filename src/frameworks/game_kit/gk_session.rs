/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! GKSession stub.

use crate::objc::{id, ClassExports, HostObject, objc_classes, nil};

struct GKSessionHostObject {}
impl HostObject for GKSessionHostObject {}

pub const CLASSES: ClassExports = objc_classes! {
    (env, this, _cmd);

    @implementation GKSession: NSObject

    // Simple stub to avoid crashing when initialized
    - (id)initWithSessionID:(id)sessionID displayName:(id)name sessionMode:(i32)mode {
        log_dbg!("[(GKSession *) {:?} initWithSessionID:{:?} displayName:{:?} sessionMode:{:?}]", this, sessionID, name, mode);
        this
    }

    @end
};
