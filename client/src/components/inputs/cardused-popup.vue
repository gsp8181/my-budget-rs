<template>
    <div class="form-group">
        <label for="message-text" class="col-form-label">Card Used:</label>
        <div class="input-group">
            <cardselect :value="value" v-on:selected-change="onUpdate"></cardselect>

            <!--max and min todo-->
        </div>
    </div>
</template>
<script>
    //TODO: css style
    //TODO: -1 will not validate
    //TODO api validates card number
    //TODO: api does not return 200 on param error
    //TODO: must have card to save?
    import cardselect from '@/components/inputs/cardselect.vue';
    export default {
        props: ['value'],
        components: {
            cardselect
        },
        methods: {
            onUpdate(newObj) {
                this.$emit('input', newObj.new);
            }
        }
    }
</script>