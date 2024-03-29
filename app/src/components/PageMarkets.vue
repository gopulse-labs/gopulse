<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { paginateposts, marketFilter } from '@/api'
import { useSlug, useFromRoute } from '@/composables'
import PostContentList from '@/components/PostContentList'
import PostSearch from '@/components/PostSearch'

// Data.
const router = useRouter()
const posts = ref([])
const market = ref('')
const slugTopic = useSlug(market)
const viewedTopic = ref('')
const filters = ref([])

const onNewPage = newposts => posts.value.push(...newposts)
const { prefetch, hasNextPage, getNextPage, loading } = paginateposts(filters, 10, onNewPage)

// Actions.
const search = () => {
    router.push("/markets/" + slugTopic.value)
}

const fetchTopicposts = () => {
    if (slugTopic.value === viewedTopic.value) return;
    posts.value = []
    viewedTopic.value = slugTopic.value
    filters.value = [marketFilter(slugTopic.value)]
    prefetch().then(getNextPage)
}
fetchTopicposts()

// Router hooks.
useFromRoute((route) => {
    market.value = route.params.market
    if (market.value) {
        fetchTopicposts()
    } else {
        posts.value = []
        viewedTopic.value = ''
    }
})
</script>

<template>
    <post-search placeholder="market" :disabled="! slugTopic" :modelValue="slugTopic" @update:modelValue="value => market = value" @search="search">
        <template #icon>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M9.243 3.03a1 1 0 01.727 1.213L9.53 6h2.94l.56-2.243a1 1 0 111.94.486L14.53 6H17a1 1 0 110 2h-2.97l-1 4H15a1 1 0 110 2h-2.47l-.56 2.242a1 1 0 11-1.94-.485L10.47 14H7.53l-.56 2.242a1 1 0 11-1.94-.485L5.47 14H3a1 1 0 110-2h2.97l1-4H5a1 1 0 110-2h2.47l.56-2.243a1 1 0 011.213-.727zM9.03 8l-1 4h2.938l1-4H9.031z" clip-rule="evenodd" />
            </svg>  
        </template>
    </post-search>
    <div v-if="viewedTopic">
        <postContent-list v-model:posts="posts" :loading="loading" :has-more="hasNextPage" @more="getNextPage"></postContent-list>
        <div v-if="!loading && posts.length === 0" class="p-8 text-gray-500 text-center">
            No posts were found in this market...
        </div>
    </div>
</template>
