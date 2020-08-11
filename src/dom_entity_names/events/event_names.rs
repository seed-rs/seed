// Comprehensive list: https://developer.mozilla.org/en-US/docs/Web/Events
make_events! {
    Cached => "cached", Error => "error", Abort => "abort", Load => "load", BeforeUnload => "beforeunload",
    Unload => "unload", Online => "online", Offline => "offline", Focus => "focus", Blur => "blur",
    Open => "open", Message => "message", Close => "close", PageHide => "pagehide",
    PageShow => "pageshow", PopState => "popstate", AnimationStart => "animationstart", AnimationEnd => "animationend",
    AnimationIteration => "animationiteration", TransitionStart => "transtionstart", TransitionEnd => "transitionend",
    TranstionRun => "transitionrun",

    Rest => "rest", Submit => "submit", BeforePrint => "beforeprint", AfterPrint => "afterprint",
    CompositionStart => "compositionstart", CompositionUpdate => "compositionupdate", CompositionEnd => "compositionend",

    FullScreenChange => "fullscreenchange", FullScreenError => "fullscreenerror", Resize => "resize",
    Scroll => "scroll", Cut => "cut", Copy => "copy", Paste => "paste",

    KeyDown => "keydown", KeyUp => "keyup",
    KeyPress => "keypress", AuxClick => "auxclick", Click => "click", ContextMenu => "contextmenu", DblClick => "dblclick",
    MouseDown => "mousedown", MouseEnter => "mouseenter", MouseLeave => "mouseleave",
    MouseMove => "mousemove", MouseOver => "mouseover", MouseOut => "mouseout", MouseUp => "mouseup",
    PointerLockChange => "pointerlockchange", PointerLockError => "pointerlockerror", Select => "select",
    Wheel => "wheel",

    PointerOver => "pointerover", PointerEnter => "pointerenter",
    PointerDown => "pointerdown", PointerMove => "pointermove", PointerUp => "pointerup",
    PointerCancel => "pointercancel", PointerOut => "pointerout", PointerLeave => "pointerleave",
    GotPointerCapture => "gotpointercapture", LostPointerCapture => "lostpointercapture",

    TouchStart => "touchstart", TouchEnd => "touchend", TouchCancel => "touchcancel", TouchMove => "touchmove",

    Drag => "drag", DragEnd => "dragend", DragEnter => "dragenter", DragStart => "dragstart", DragLeave => "dragleave",
    DragOver => "dragover", Drop => "drop",

    AudioProcess => "audioprocess", CanPlay => "canplay", CanPlayThrough => "canplaythrough", Complete => "complete",
    DurationChange => "durationchange", Emptied => "emptied", Ended => "ended", LoadedData => "loadeddata",
    LoadedMetaData => "loadedmetadata", Pause => "pause", Play => "play", Playing => "playing", RateChange => "ratechange",
    Seeked => "seeked", Seeking => "seeking", Stalled => "stalled", Suspend => "suspend", TimeUpdate => "timeupdate",
    VolumeChange => "volumechange", Waiting => "waiting",

    LoadEnd => "loadend",LoadStart => "loadstart" , Timeout => "timeout",

    Change => "change",Storage => "storage",

    Checking => "checking",Downloading=> "downloading", NoUpdate => "noupdate",Obselete => "obsolete", UpdateReady => "updateready",

    Broadcast => "broadcast", CheckBoxStateChange => "CheckBoxStateChange", HasChange => "haschange",  Input => "input" , RadioStateChange => "RadioStateChange",
    ReadyStateChange => "readystatechange",ValueChange => "ValueChange",

    Invalid => "invalid",Show => "show",

    SVGAbort => "SVGAbort", SVGError => "SVGError", SVGLoad => "SVGLoad", SVGResize => "SVGResize",SVGScroll => "SVGScroll"  , SVGUnload => "SVGUnload",

    Blocked => "blocked", Success => "success", UpgradeNeeded => "upgradeneeded", VersionChange => "versionchange",

    AfterScriptExecute => "afterscriptexecute", BeforeScriptExecute => "beforescriptexecute",
    DOMMenuItemActive => "DOMMenuItemActive", DOMMenuteItemInactive => "DOMMEnuItemInactive",

    PopupHidden=>"popuphidden",PopupHiding => "popuphiding", PopupShowing => "popupshowing", PopupShown => "popupshown",

    VisibilityChange => "visibilitychange",

    ChargingChange => "chargingchange",
    ChargingTimeChange =>"chargingtimechange",
    DischargingTimeChange => "dischargingtimechange",

    Connected => "connected",

    StateChange => "statechange",

    DeviceMotion => "devicemotion",
    DeviceOrientation => "deviceorientation",
    OrientationChange => "orientationchange",
    SmartCardInsert => "smartcard-insert",
    SmartCardRemove => "smartcard-remove",

    SelectionChange => "selectionchange"
}
