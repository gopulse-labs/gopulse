<template>
    <div class="p-8 text-gray-500 text-center">
        <p>404 — Not Found</p>
        <router-link :to="{ name: 'Feed' }" class="block text-blue-500 hover:underline mt-2">
            Go back home
        </router-link>
    </div>
</template>
