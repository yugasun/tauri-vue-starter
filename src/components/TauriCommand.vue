<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/tauri';

invoke('hello', { name: 'tauri-vue-starter' }).then((res) => {
    console.log(res);
});
</script>
<template>
    <div class="tauri-command"></div>
</template>
<style lang="scss" scoped></style>
