/*
 * Copyright Rodolphe Breard (2015-2017)
 * Author: Rodolphe Breard (2015-2017)
 *
 * This software is a computer library whose purpose is to offer a
 * collection of tools for user authentication.
 *
 * This software is governed by the CeCILL  license under French law and
 * abiding by the rules of distribution of free software.  You can  use,
 * modify and/ or redistribute the software under the terms of the CeCILL
 * license as circulated by CEA, CNRS and INRIA at the following URL
 * "http://www.cecill.info".
 *
 * As a counterpart to the access to the source code and  rights to copy,
 * modify and redistribute granted by the license, users are provided only
 * with a limited warranty  and the software's author,  the holder of the
 * economic rights,  and the successive licensors  have only  limited
 * liability.
 *
 * In this respect, the user's attention is drawn to the risks associated
 * with loading,  using,  modifying and/or developing or reproducing the
 * software by the user in light of its specific status of free software,
 * that may mean  that it is complicated to manipulate,  and  that  also
 * therefore means  that it is reserved for developers  and  experienced
 * professionals having in-depth computer knowledge. Users are therefore
 * encouraged to load and test the software's suitability as regards their
 * requirements in conditions enabling the security of their systems and/or
 * data to be ensured and,  more generally, to use and operate it in the
 * same conditions as regards security.
 *
 * The fact that you are presently reading this means that you have had
 * knowledge of the CeCILL license and that you accept its terms.
 */

//!
//! [![Build Status](https://api.travis-ci.org/breard-r/libreauth.svg?branch=master)](https://travis-ci.org/breard-r/libreauth)
//! [![LibreAuth on crates.io](https://img.shields.io/crates/v/libreauth.svg)](https://crates.io/crates/libreauth)
//! [![LibreAuth on docs.rs](https://docs.rs/libreauth/badge.svg)](https://docs.rs/libreauth/)
//!
//! LibreAuth is a collection of tools for user authentication.
//!
//!

extern crate argon2;
extern crate hmac;
extern crate pbkdf2;
extern crate sha1;
extern crate sha2;

extern crate base32;
extern crate base64;
extern crate hex;
extern crate rand;
extern crate time;
#[macro_use]
extern crate nom;

pub mod oath;
pub mod pass;
pub mod key;

#[cfg(feature = "cbindings")]
extern crate libc;
