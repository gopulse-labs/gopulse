<script setup>
import { computed, ref } from 'vue'
import { useAutoresizeTextarea } from '@/composables'
import { createUser, updateUser, fetchUser } from '@/api'
import { useWallet } from 'solana-wallets-vue'
import { useAnchorWallet } from 'solana-wallets-vue'

const wallet = useAnchorWallet()

// Form data.
const name = ref('')
const avatar = ref('')
const username = ref('')
const useravatar = ref('')
const profileExists = ref()

// Auto-resize the content's textarea.
const textarea = ref()
useAutoresizeTextarea(textarea)

// Character limit / count-down.
// const characterLimit = useCountCharacterLimit(content, 420)
// const characterLimitColour = computed(() => {
//     if (characterLimit.value < 0) return 'text-red-500'
//     if (characterLimit.value <= 10) return 'text-yellow-500'
//     return 'text-gray-400'
// })

// Permissions.
const { connected } = useWallet()
const canPostContent = computed(() => name.value && avatar.value)

fetchUser(wallet.value.publicKey).then(res => {
    profileExists.value = true
    username.value = res.name
    useravatar.value = res.avatar
}).catch(() => {
    profileExists.value = false
})

// Actions.
const setAccount = async () => {
    console.log(name.value, avatar.value)
    await createUser(name.value, avatar.value)
    name.value = ''
    avatar.value = ''
}

const updateAccount = async () => {
    console.log(name.value, avatar.value)
    await updateUser(name.value, avatar.value)
    name.value = ''
    avatar.value = ''
}

</script>

<template>
    <div v-if="connected" class="px-8 py-4 border-b">

        <div class="flex flex-wrap items-center justify-between -m-2">

            <!-- Topic field. -->
            <div class="relative m-2 mr-4">
                <input
                    type="text"
                    placeholder="Username"
                    class="text-blue-800 rounded-full pl-5 pr-1 py-2 bg-gray-500"
                    :value="name"
                    @input="name = $event.target.value"
                >
               
            </div>

            <div class="relative m-2 mr-4">
                <input
                    type="text"
                    placeholder="Avatar"
                    class="text-blue-800 rounded-full pl-5 pr-1 py-2 bg-gray-500"
                    :value="avatar"
                    @input="avatar = $event.target.value"
                >
               
            </div>
            
            <div class="flex items-center space-x-6 m-2 ml-auto">

                <button v-if="!profileExists"
                    class="text-white px-4 py-2 rounded-full font-semibold" :disabled="! canPostContent"
                    :class="canPostContent ? 'bg-blue-800' : 'bg-blue-800 cursor-not-allowed'"
                    @click="setAccount"
                >
                    Set Account
                </button>
                

                <!-- PostContent button. -->
                <button v-if="profileExists"
                    class="text-white px-4 py-2 rounded-full font-semibold" :disabled="! canPostContent"
                    :class="canPostContent ? 'bg-blue-800' : 'bg-blue-800 cursor-not-allowed'"
                    @click="updateAccount"
                >
                    Update Account
                </button>
            </div>
        </div>
    </div>

    <div v-else class="px-8 py-4 bg-gray-50 text-gray-500 text-center border-b">
        Connect your wallet to start posting...
    </div>
</template>