// Copyright IBM Corporation 2021

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use] extern crate rocket;
use std::path::PathBuf;

#[get("/<_file..>")]
fn hello(_file: PathBuf) -> &'static str {
    "This is a rust web app."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
}
