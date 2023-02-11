use libc;

/// Used for checking API/ABI mismatches that can break libretro implementations
/// It is not incremented for compatible changes to the API.
pub const API_VERSION: libc::c_uint = 1;

/// Libretro's fundamental device abstractions.
/// Libretro's input system consists of some standardized device types,
/// such as a joypad (with/without analog), mouse, keyboard, lightgun
/// and a pointer.
///
///  The functionality of these devices are fixed, and individual cores
/// map their own concept of a controller to libretro's abstractions.
/// This makes it possible for frontends to map the abstract types to a
/// real input device, and not having to worry about binding input
/// correctly to arbitrary controller layouts.
pub const DEVICE_TYPE_SHIFT: libc::c_uint = 8;
pub const DEVICE_MASK: libc::c_uint = (1 << DEVICE_TYPE_SHIFT) - 1;
// todo: device subclas

/// Input disabled
pub const DEVICE_NONE: libc::c_uint = 0;

/// The JOYPAD is called RetroPad. It is essentially a Super Nintendo
/// controller, but with additional L2/R2/L3/R3 buttons, similar to a
/// PS1 DualShock.
pub const DEVICE_JOYPAD: libc::c_uint = 1;

/// The mouse is a simple mouse, similar to Super Nintendo's mouse.
/// X and Y coordinates are reported relatively to last poll (poll callback).
/// It is up to the libretro implementation to keep track of where the mouse
/// pointer is supposed to be on the screen.
/// The frontend must make sure not to interfere with its own hardware
/// mouse pointer.
pub const DEVICE_MOUSE: libc::c_uint = 2;

/// KEYBOARD device lets one poll for raw key pressed.
/// It is poll based, so input callback will return with the current
/// pressed state.
/// For event/text based keyboard input, see
/// RETRO_ENVIRONMENT_SET_KEYBOARD_CALLBACK.
pub const DEVICE_KEYBOARD: libc::c_uint = 3;

/// LIGHTGUN device is similar to Guncon-2 for PlayStation 2.
/// It reports X/Y coordinates in screen space (similar to the pointer)
/// in the range [-0x8000, 0x7fff] in both axes, with zero being center and
/// -0x8000 being out of bounds.
/// As well as reporting on/off screen state. It features a trigger,
/// start/select buttons, auxiliary action buttons and a
/// directional pad. A forced off-screen shot can be requested for
/// auto-reloading function in some games.
pub const DEVICE_LIGHTGUN: libc::c_uint = 4;

/// The ANALOG device is an extension to JOYPAD (RetroPad).
/// Similar to DualShock2 it adds two analog sticks and all buttons can
/// be analog. This is treated as a separate device type as it returns
/// axis values in the full analog range of [-0x7fff, 0x7fff],
/// although some devices may return -0x8000.
/// Positive X axis is right. Positive Y axis is down.
/// Buttons are returned in the range [0, 0x7fff].
/// Only use ANALOG type when polling for analog values.
pub const DEVICE_ANALOG: libc::c_uint = 5;

/// Abstracts the concept of a pointing mechanism, e.g. touch.
/// This allows libretro to query in absolute coordinates where on the
/// screen a mouse (or something similar) is being placed.
/// For a touch centric device, coordinates reported are the coordinates
/// of the press.
///
/// Coordinates in X and Y are reported as:
/// [-0x7fff, 0x7fff]: -0x7fff corresponds to the far left/top of the screen,
/// and 0x7fff corresponds to the far right/bottom of the screen.
/// The "screen" is here defined as area that is passed to the frontend and
/// later displayed on the monitor.
///
/// The frontend is free to scale/resize this screen as it sees fit, however,
/// (X, Y) = (-0x7fff, -0x7fff) will correspond to the top-left pixel of the
/// game image, etc.
///
/// To check if the pointer coordinates are valid (e.g. a touch display
/// actually being touched), PRESSED returns 1 or 0.
///
/// If using a mouse on a desktop, PRESSED will usually correspond to the
/// left mouse button, but this is a frontend decision.
/// PRESSED will only return 1 if the pointer is inside the game screen.
///
/// For multi-touch, the index variable can be used to successively query
/// more presses.
/// If index = 0 returns true for _PRESSED, coordinates can be extracted
/// with _X, _Y for index = 0. One can then query _PRESSED, _X, _Y with
/// index = 1, and so on.
/// Eventually _PRESSED will return false for an index. No further presses
/// are registered at this point
pub const DEVICE_POINTER: libc::c_uint = 6;

/// Buttons for the RetroPad (JOYPAD).
/// The placement of these is equivalent to placements on the
/// Super Nintendo controller.
/// L2/R2/L3/R3 buttons correspond to the PS1 DualShock.
/// Also used as id values for RETRO_DEVICE_INDEX_ANALOG_BUTTON
pub const DEVICE_ID_JOYPAD_B: libc::c_uint = 0;
pub const DEVICE_ID_JOYPAD_Y: libc::c_uint = 1;
pub const DEVICE_ID_JOYPAD_SELECT: libc::c_uint = 2;
pub const DEVICE_ID_JOYPAD_START: libc::c_uint = 3;
pub const DEVICE_ID_JOYPAD_UP: libc::c_uint = 4;
pub const DEVICE_ID_JOYPAD_DOWN: libc::c_uint = 5;
pub const DEVICE_ID_JOYPAD_LEFT: libc::c_uint = 6;
pub const DEVICE_ID_JOYPAD_RIGHT: libc::c_uint = 7;
pub const DEVICE_ID_JOYPAD_A: libc::c_uint = 8;
pub const DEVICE_ID_JOYPAD_X: libc::c_uint = 9;
pub const DEVICE_ID_JOYPAD_L: libc::c_uint = 10;
pub const DEVICE_ID_JOYPAD_R: libc::c_uint = 11;
pub const DEVICE_ID_JOYPAD_L2: libc::c_uint = 12;
pub const DEVICE_ID_JOYPAD_R2: libc::c_uint = 13;
pub const DEVICE_ID_JOYPAD_L3: libc::c_uint = 14;
pub const DEVICE_ID_JOYPAD_R3: libc::c_uint = 15;

pub const DEVICE_ID_JOYPAD_MASK: libc::c_uint = 256;

/// Index / Id values for ANALOG device
pub const DEVICE_INDEX_ANALOG_LEFT: libc::c_uint = 0;
pub const DEVICE_INDEX_ANALOG_RIGHT: libc::c_uint = 1;
pub const DEVICE_INDEX_ANALOG_BUTTON: libc::c_uint = 2;
pub const DEVICE_ID_ANALOG_X: libc::c_uint = 0;
pub const DEVICE_ID_ANALOG_Y: libc::c_uint = 1;

/// Id values for MOUSE
pub const DEVICE_ID_MOUSE_X: libc::c_uint = 0;
pub const DEVICE_ID_MOUSE_Y: libc::c_uint = 1;
pub const DEVICE_ID_MOUSE_LEFT: libc::c_uint = 2;
pub const DEVICE_ID_MOUSE_RIGHT: libc::c_uint = 3;
pub const DEVICE_ID_MOUSE_WHEELUP: libc::c_uint = 4;
pub const DEVICE_ID_MOUSE_WHEELDOWN: libc::c_uint = 5;
pub const DEVICE_ID_MOUSE_MIDDLE: libc::c_uint = 6;
pub const DEVICE_ID_MOUSE_HORIZ_WHEELUP: libc::c_uint = 7;
pub const DEVICE_ID_MOUSE_HORIZ_WHEELDOWN: libc::c_uint = 8;
pub const DEVICE_ID_MOUSE_BUTTON_4: libc::c_uint = 9;
pub const DEVICE_ID_MOUSE_BUTTON_5: libc::c_uint = 10;

/// Id values for LIGHTGUN
pub const DEVICE_ID_LIGHTGUN_SCREEN_X: libc::c_uint = 13;
pub const DEVICE_ID_LIGHTGUN_SCREEN_Y: libc::c_uint = 14;
pub const DEVICE_ID_LIGHTGUN_IS_OFFSCREEN: libc::c_uint = 15;
pub const DEVICE_ID_LIGHTGUN_TRIGGER: libc::c_uint = 2;
pub const DEVICE_ID_LIGHTGUN_RELOAD: libc::c_uint = 16;
pub const DEVICE_ID_LIGHTGUN_AUX_A: libc::c_uint = 3;
pub const DEVICE_ID_LIGHTGUN_AUX_B: libc::c_uint = 4;
pub const DEVICE_ID_LIGHTGUN_START: libc::c_uint = 6;
pub const DEVICE_ID_LIGHTGUN_SELECT: libc::c_uint = 7;
pub const DEVICE_ID_LIGHTGUN_AUX_C: libc::c_uint = 8;
pub const DEVICE_ID_LIGHTGUN_DPAD_UP: libc::c_uint = 9;
pub const DEVICE_ID_LIGHTGUN_DPAD_DOWN: libc::c_uint = 10;
pub const DEVICE_ID_LIGHTGUN_DPAD_LEFT: libc::c_uint = 11;
pub const DEVICE_ID_LIGHTGUN_DPAD_RIGHT: libc::c_uint = 12;

/// deprecated
pub const DEVICE_ID_LIGHTGUN_X: libc::c_uint = 0;
pub const DEVICE_ID_LIGHTGUN_Y: libc::c_uint = 1;
pub const DEVICE_ID_LIGHTGUN_CURSOR: libc::c_uint = 3;
pub const DEVICE_ID_LIGHTGUN_TURBO: libc::c_uint = 4;
pub const DEVICE_ID_LIGHTGUN_PAUSE: libc::c_uint = 5;

/// Id values for POINTER
pub const DEVICE_ID_POINTER_X: libc::c_uint = 0;
pub const DEVICE_ID_POINTER_Y: libc::c_uint = 1;
pub const DEVICE_ID_POINTER_PRESSED: libc::c_uint = 2;
pub const DEVICE_ID_POINTER_COUNT: libc::c_uint = 3;

/// Returned from retro_get_region()
pub const REGION_NTSC: libc::c_uint = 0;
pub const REGION_PAL: libc::c_uint = 1;
