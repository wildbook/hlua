#!/bin/bash

BINDGEN_VERSION=$(bindgen --version | grep -v -e '^cargo')

bindgen -o src/ffi.rs \
 --raw-line "/// Generated with: ${BINDGEN_VERSION}" \
 --whitelist-var "LUA.*" \
 --whitelist-var "LUAJIT.*" \
 --whitelist-type "lua_.*" \
 --whitelist-type "luaL_.*" \
 --whitelist-function "lua_.*" \
 --whitelist-function "luaL_.*" \
 --whitelist-function "luaJIT.*" \
 --whitelist-function "luaopen.*" \
 --ctypes-prefix "libc" \
 --use-core \
 --impl-debug \
 --size_t-is-usize \
 --default-macro-constant-type signed \
 ffi.h -- -I lua/src

sed -i -e 's/pub fn \(luaJIT_[^\(]*\)/\/\/\/ <https:\/\/luajit.org\/ext_c_api.html> \n pub fn \1/' src/ffi.rs
sed -i -e 's/pub fn \(lua_[^ \(]*\)/\/\/\/ <https\:\/\/www.lua.org\/manual\/5.4\/manual.html#\1> \n pub fn \1/' src/ffi.rs
sed -i -e 's/pub fn \(luaL_[^ \(]*\)/\/\/\/ <https\:\/\/www.lua.org\/manual\/5.4\/manual.html#\1> \n pub fn \1/' src/ffi.rs
sed -i -e 's/pub fn \(luaopen[^ \(]*\)/\/\/\/ <https\:\/\/www.lua.org\/manual\/5.4\/manual.html#\1> \n pub fn \1/' src/ffi.rs
sed -i -e 's/pub type \(lua_[^ \=]*\)/\/\/\/ <https\:\/\/www.lua.org\/manual\/5.4\/manual.html#\1> \n pub type \1/' src/ffi.rs
sed -i -e 's/pub struct \(lua_[^ \{]*\)/\/\/\/ <https\:\/\/www.lua.org\/manual\/5.4\/manual.html#\1> \n pub struct \1/' src/ffi.rs
sed -i -e 's/pub struct \(luaL_[^ \{]*\)/\/\/\/ <https\:\/\/www.lua.org\/manual\/5.4\/manual.html#\1> \n pub struct \1/' src/ffi.rs

cargo fmt