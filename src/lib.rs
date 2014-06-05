#![crate_id = "lua"]
#![crate_type = "lib"]
#![comment = "Lua bindings for Rust"]
#![license = "MIT"]
#![allow(visible_private_types)]
#![feature(macro_rules)]

extern crate libc;
extern crate std;

mod liblua;
mod functions;
mod tables;
mod values;

/**
 * Main object of the library
 */
pub struct Lua {
	lua: *mut liblua::lua_State
}

/**
 * Object which allows access to a Lua variable
 */
pub struct VariableAccessor<'a, TVariableLocation> {
	lua: &'a mut Lua,
	location: TVariableLocation
}

/**
 * Should be implemented by whatever type is pushable on the Lua stack
 */
pub trait Pushable {
	fn push_to_lua(&self, &Lua);
}

impl<'a, T:Pushable> Pushable for &'a T {
	fn push_to_lua(&self, lua: &Lua) {
		(*self).push_to_lua(lua)
	}
}

/**
 * Should be implemented by whatever type can be read from the Lua stack
 */
pub trait Readable {
	/**
	 * # Arguments
	 *  * `lua` - The Lua object to read from
	 *  * `index` - The index on the stack to read from
	 */
	fn read_from_lua(lua: &Lua, index: i32) -> Option<Self>;
}

/**
 * Types that can be indices in Lua tables
 */
pub trait Index: Pushable + Readable {
	fn lua_set_global<T: Pushable>(&self, lua: &Lua, value: T) {
		unsafe { liblua::lua_pushglobaltable(lua.lua); }
		value.push_to_lua(lua);
		self.push_to_lua(lua);
		unsafe { liblua::lua_settable(lua.lua, -3); }
		unsafe { liblua::lua_pop(lua.lua, 1); }
	}

	fn lua_get_global<T: Readable>(&self, lua: &Lua) -> Option<T> {
		unsafe { liblua::lua_pushglobaltable(lua.lua); }
		self.push_to_lua(lua);
		unsafe { liblua::lua_gettable(lua.lua, -2); }
		Readable::read_from_lua(lua, -1)
	}
}

/**
 * Object which can store variables
 */
trait Dropbox<I: Index, V: Pushable> {
	fn store(&self, &Lua, &I, V) -> Result<(), &'static str>;
}

/**
 * Object which you can read variables from
 */
trait Readbox<I: Index, V: Readable> {
	fn read(&self, &Lua, &I) -> Option<V>;
}

struct VariableLocation<I, Prev> {
	index: I,
	prev: Prev
}

/**
 * Represents the global variables
 */
pub struct Globals;

/**
 * Error that can happen when executing Lua code
 */
pub enum ExecutionError {
	SyntaxError(String),
	ExecError(String)
}


extern "C" fn alloc(_ud: *mut libc::c_void, ptr: *mut libc::c_void, _osize: libc::size_t, nsize: libc::size_t) -> *mut libc::c_void {
    unsafe {
        if nsize == 0 {
            libc::free(ptr as *mut libc::c_void);
            std::ptr::mut_null()
        } else {
            libc::realloc(ptr, nsize)
        }
    }
}

impl Lua {
	/**
	 * Builds a new Lua context
	 * # Failure
	 * The function fails if lua_newstate fails (which indicates lack of memory)
	 */
	pub fn new() -> Lua {
		let lua = unsafe { liblua::lua_newstate(alloc, std::ptr::mut_null()) };
		if lua.is_null() {
			fail!("lua_newstate failed");
		}

		Lua { lua: lua }
	}

	/**
	 * Executes some Lua code on the context
	 */
	pub fn execute<T: Readable>(&mut self, code: &String) -> Result<T, ExecutionError> {
		unimplemented!()
	}

	pub fn access<'a, I: Index>(&'a mut self, index: I) -> VariableAccessor<'a, VariableLocation<I, Globals>> {
		VariableAccessor {
			lua: self,
			location: VariableLocation { index: index, prev: Globals }
		}
	}

	pub fn get<I: Index, V: Readable>(&mut self, index: I) -> Option<V> {
		self.access(index).get()
	}

	pub fn set<I: Index, V: Pushable>(&mut self, index: I, value: V) -> Result<(), &'static str> {
		self.access(index).set(value)
	}
}

impl<'a, I: Index, V: Pushable, DB: Dropbox<I, V>> VariableAccessor<'a, VariableLocation<I, DB>> {
	pub fn set(&mut self, value: V) -> Result<(), &'static str> {
		let loc = &self.location;
		loc.prev.store(self.lua, &loc.index, value)
	}
}

impl<'a, I: Index, V: Readable, RB: Readbox<I, V>> VariableAccessor<'a, VariableLocation<I, RB>> {
	pub fn get(&self) -> Option<V> {
		let loc = &self.location;
		loc.prev.read(self.lua, &loc.index)
	}
}

impl<I: Index, V: Pushable> Dropbox<I, V> for Globals {
	fn store(&self, lua: &Lua, index: &I, value: V) -> Result<(), &'static str> {
		index.lua_set_global(lua, value);
		Ok(())
	}
}

impl<I: Index, V: Readable> Readbox<I, V> for Globals {
	fn read(&self, lua: &Lua, index: &I) -> Option<V> {
		index.lua_get_global(lua)
	}
}

/*impl<'a, TIndex: Index, TValue: Pushable, TIndex2: Index, TPrev: Readbox<TIndex2, >> Dropbox<TIndex, TValue> for VariableLocation<'a, TIndex2, TPrev> {
	fn store(&self, lua: &Lua, index: &TIndex, value: TValue) -> Result<(), &'static str> {
		match self.prev.read(lua, self.index) {
			Some(_) => (),
			None => return Err("Could not load the table")
		}
		index.push_to_lua(lua);
		value.push_to_lua(lua);
		unsafe { liblua::lua_settable(lua.lua, -3); }
		Ok(())
	}
}*/