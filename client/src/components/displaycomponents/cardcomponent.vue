<template>
    <div class="field">
        <cardselect :value="value" v-on:selected-change="onUpdate" :selectedcard="this.text"></cardselect>
    </div>
</template>
<script>
    import cardselect from '@/components/inputs/cardselect.vue';
    export default {
        components: {
            cardselect
        },
        props: ['name', 'text', 'id'],
        methods: {
            onUpdate(newObj) {
                this.$emit('on-save', {
                    name: this.name,
                    text: newObj.new,
                    id: this.id
                })
            }
        }
    }
</script>
<style scoped>
    /*todo: implement*/
</style>