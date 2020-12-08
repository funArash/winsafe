const_type! { FF, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfPitchAndFamily`, used with
	/// [`PITCH`](crate::co::PITCH).

	DONTCARE, 0 << 4
	ROMAN, 1 << 4
	SWISS, 2 << 4
	MODERN, 3 << 4
	SCRIPT, 4 << 4
	DECORATIVE, 5 << 4
}

const_type! { FORMAT_MESSAGE, u32,
	/// [`FormatMessage`](crate::co::ERROR::FormatMessage) `dwFlags`.

	ALLOCATE_BUFFER, 0x00000100
	ARGUMENT_ARRAY, 0x00002000
	FROM_HMODULE, 0x00000800
	FROM_STRING, 0x00000400
	FROM_SYSTEM, 0x00001000
	IGNORE_INSERTS, 0x00000200
	MAX_WIDTH_MASK, 0x000000ff
}