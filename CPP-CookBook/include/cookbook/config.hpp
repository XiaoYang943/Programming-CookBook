#pragma once

#if !defined(COOKBOOK_CPLUSPLUS)
#if defined(_MSVC_LANG)
#define COOKBOOK_CPLUSPLUS _MSVC_LANG
#else
#define COOKBOOK_CPLUSPLUS __cplusplus
#endif
#endif

#if !defined(COOKBOOK_CPP20)
#if COOKBOOK_CPLUSPLUS >= 202002L
#define COOKBOOK_CPP20 1
#else
#define COOKBOOK_CPP20 0
#endif
#endif

#if !defined(COOKBOOK_CPP23)
#if COOKBOOK_CPLUSPLUS >= 202302L || (defined(_MSVC_LANG) && COOKBOOK_CPLUSPLUS >= 202004L)
#define COOKBOOK_CPP23 1
#else
#define COOKBOOK_CPP23 0
#endif
#endif
