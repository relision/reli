// Copyright (c) 2015 by Stacy Prowell.  All rights reserved.
//
// Licensed under the BSD 2-Clause license.  See the file LICENSE
// that is part of this distribution.  This file may not be copied,
// modified, or distributed except according to those terms.extern

//! Define the special root term.

pub trait Root;

struct iRoot;

impl Root for iRoot;

impl Term for iRoot {
	fn get_type() -> Rc<Root> {
		Rc::new(self)
	}
}

static let known_root = iRoot;
