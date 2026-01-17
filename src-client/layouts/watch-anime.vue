<template>
  <div class="min-h-screen bg-gray-950 text-white">
    <nav
      class="flex items-center justify-between px-6 py-4 bg-black/80 fixed w-full z-50"
    >
      <div class="flex items-center gap-3">
        <span class="text-2xl font-bold">🎬 Watch Anime</span>
      </div>

      <div class="flex gap-4">
        <NuxtLink class="hover:text-blue-400" to="/watch-anime">Home</NuxtLink>
        <NuxtLink class="hover:text-blue-400" to="/watch-anime/spotlight"
          >Spotlight</NuxtLink
        >
        <NuxtLink class="hover:text-blue-400" to="/watch-anime/topten"
          >Top Ten</NuxtLink
        >
        <NuxtLink class="hover:text-blue-400" to="/watch-anime/list/top-airing"
          >Top Airing</NuxtLink
        >
        <NuxtLink
          class="hover:text-blue-400"
          to="/watch-anime/list/most-popular"
          >Most Popular</NuxtLink
        >
        <NuxtLink class="hover:text-blue-400" to="/watch-anime/search"
          >Search</NuxtLink
        >
      </div>
    </nav>

    <main class="pt-20">
      <slot />
    </main>
  </div>
</template>
