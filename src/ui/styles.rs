// Header
pub const HEADER_H1: &str = "text-5xl font-black tracking-tighter mb-4 bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 bg-clip-text text-transparent";
pub const HEADER_CONTAINER: &str = "max-w-7xl mx-auto mb-4 text-center"; // Compacted
pub const HEADER_SUBTITLE: &str = "text-gray-400 text-sm uppercase tracking-widest font-mono mb-6";

pub const BTN_BASE: &str = "group relative border-2 transition-all duration-300 flex items-center justify-center font-bold";
pub const BTN_SELECTED: &str = "bg-indigo-600 border-white shadow-[0_0_20px_rgba(79,70,229,0.6)] scale-110";
pub const BTN_NORMAL: &str = "bg-neutral-800 border-neutral-700 hover:border-indigo-500 text-neutral-400 hover:text-white";
pub const BTN_NONE_SELECTED: &str = "bg-emerald-600 border-white text-white shadow-[0_0_15px_rgba(16,185,129,0.4)]";
pub const BTN_NONE_NORMAL: &str = "bg-neutral-800 border-neutral-700 text-neutral-500";

pub const APP_CONTAINER: &str = "min-h-screen bg-[#2a2a2a] text-white font-sans selection:bg-indigo-500 p-2 md:p-6"; 

pub const BTN_WEIGHT_BASE: &str = "flex-1 py-4 rounded-2xl border-2 font-black transition-all";
pub const BTN_WEIGHT_HEAVY: &str = "bg-red-500 border-white text-white shadow-lg";
pub const BTN_WEIGHT_HEAVY_INACTIVE: &str = "bg-neutral-800 border-neutral-700 text-neutral-500 hover:bg-neutral-700";
pub const BTN_WEIGHT_LIGHT: &str = "bg-blue-500 border-white text-white shadow-lg";
pub const BTN_WEIGHT_LIGHT_INACTIVE: &str = "bg-neutral-800 border-neutral-700 text-neutral-500 hover:bg-neutral-700";

// Panels & Inputs
pub const PANEL_BASE: &str = "bg-[#1a1a1a] p-6 rounded-2xl border border-neutral-800";
pub const PANEL_HEADER: &str = "text-neutral-400 font-bold text-xs uppercase tracking-widest mb-4";
pub const INPUT_BASE: &str = "bg-neutral-800 text-white rounded px-2 py-1 outline-none focus:ring-2 ring-emerald-500";
pub const PANEL_CONTAINER: &str = "max-w-7xl mx-auto mb-4";
pub const OVERLAY_BOX: &str = "bg-[#2a2a2a] p-12 rounded-3xl text-center max-w-md";
pub const OVERLAY_BTN: &str = "px-8 py-3 bg-white text-black font-bold rounded-full hover:scale-105 transition-transform";

// Coin Buttons (Game Mode)
pub const COIN_BTN_BASE: &str = "w-10 h-10 rounded-full font-bold text-sm transition-all transform hover:scale-110 active:scale-95";
pub const COIN_BTN_SELECTED: &str = "bg-emerald-500 text-black shadow-lg shadow-emerald-500/50 scale-110 ring-2 ring-white";
pub const COIN_BTN_NORMAL: &str = "bg-amber-400 text-amber-900 border-2 border-amber-600 shadow-md";

// Coin Buttons (Solver Mode)
pub const COIN_BTN_SIZE_LABEL: &str = "px-4 md:px-6 h-10 md:h-16 rounded-full uppercase tracking-tight";
pub const COIN_BTN_SIZE_DEFAULT: &str = "min-w-[40px] w-10 h-10 md:min-w-[64px] md:w-16 md:h-16 aspect-square rounded-full text-lg md:text-xl";
pub const COIN_BTN_BADGE: &str = "absolute -top-2 -right-2 w-6 h-6 bg-white rounded-full flex items-center justify-center text-xs text-indigo-600 font-black border-2 border-indigo-600 z-10";

// Actions
pub const WEIGH_BTN: &str = "px-8 py-3 bg-gradient-to-r from-blue-600 to-indigo-600 text-white font-bold rounded-xl shadow-lg hover:shadow-indigo-500/30 transform hover:-translate-y-1 transition-all disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none disabled:shadow-none";

// Interactive Scale Drop Zones
pub const DROP_ZONE_LEFT: &str = "absolute top-[140px] left-[20px] w-[60px] h-[60px] cursor-pointer hover:bg-white/10 rounded-full flex items-center justify-center transition-colors border-2 border-transparent hover:border-white/20";
pub const DROP_ZONE_RIGHT: &str = "absolute top-[140px] left-[220px] w-[60px] h-[60px] cursor-pointer hover:bg-white/10 rounded-full flex items-center justify-center transition-colors border-2 border-transparent hover:border-white/20";
pub const DROP_ZONE_ACTIVE: &str = "animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-20";

pub const WEIGH_BTN_DISABLED: &str = "px-8 py-3 bg-neutral-700 text-neutral-500 font-bold rounded-xl cursor-not-allowed opacity-50";

pub const BTN_CLEAR_PRIMARY: &str = "px-6 py-3 bg-amber-600 text-white rounded-xl hover:bg-amber-500 transition-colors font-bold text-sm shadow-lg shadow-amber-900/20";
pub const BTN_CLEAR_NORMAL: &str = "px-6 py-3 bg-neutral-700 text-neutral-300 rounded-xl hover:bg-neutral-600 transition-colors font-bold text-sm";


// Drag & Drop
pub const DROPTARGET: &str = "absolute w-[60px] h-[60px] cursor-pointer hover:bg-white/10 rounded-full flex items-center justify-center transition-colors border-2 border-transparent hover:border-white/20";

// Mode Switcher
pub const MODE_SWITCHER_CONTAINER: &str = "flex justify-end p-4 gap-2 z-50 relative";
pub const MODE_BTN_ACTIVE: &str = "px-3 py-1 bg-blue-600 text-white rounded shadow";
pub const MODE_BTN_INACTIVE: &str = "px-3 py-1 bg-gray-700 text-gray-300 rounded hover:bg-gray-600";

// Inputs & Controls
pub const INPUT_RANGE: &str = "w-full h-2 bg-neutral-700 rounded-lg appearance-none cursor-pointer accent-emerald-500";
pub const INPUT_NUMBER: &str = "w-16 bg-neutral-700 text-white rounded px-2 py-1 text-center no-spinner outline-none focus:ring-2 ring-emerald-500 border border-neutral-600 focus:border-emerald-500";
pub const INPUT_INVALID: &str = "border-red-500 focus:border-red-500 focus:ring-red-500 bg-red-900/20";
pub const BTN_RESTART: &str = "w-full py-2 bg-neutral-800 hover:bg-neutral-700 text-neutral-300 rounded-lg font-mono text-xs transition-colors border border-neutral-700";
pub const BTN_RESTART_PENDING: &str = "w-full py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg font-mono text-xs transition-all shadow-lg shadow-emerald-900/20 animate-pulse font-bold";

// History
pub const HISTORY_ITEM: &str = "p-3 bg-neutral-800/50 rounded-lg border border-neutral-700 flex items-center justify-between";

// Guessing
pub const GUESS_BTN_HEAVY: &str = "py-2 bg-red-900/30 text-red-400 border border-red-900/50 rounded hover:bg-red-900/50 transition-colors uppercase font-bold text-xs";
pub const GUESS_BTN_LIGHT: &str = "py-2 bg-blue-900/30 text-blue-400 border border-blue-900/50 rounded hover:bg-blue-900/50 transition-colors uppercase font-bold text-xs";
pub const GUESS_BTN_NONE: &str = "w-full mt-2 py-2 bg-emerald-900/30 text-emerald-400 border border-emerald-900/50 rounded hover:bg-emerald-900/50 transition-colors uppercase font-bold text-xs";

// Result
pub const RESULT_TITLE_BASE: &str = "text-5xl font-black mb-4 tracking-tighter";

// Overlays
pub const OVERLAY_CONTAINER: &str = "fixed inset-0 z-[100] flex items-center justify-center bg-black/80 backdrop-blur-sm";
