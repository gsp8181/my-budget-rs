<template>
    <div class="form-group">
        <label for="recipient-name" class="col-form-label">{{name}}:</label>
        <input type="text" :placeholder="placeholder" class="form-control inputmodal-elem" name="name"
            v-bind:value="value" v-on:input="$emit('input', $event.target.value)" required>
    </div>
</template>
<script>
    export default {
        name: 'name-popup',
        props: ['name', 'placeholder', 'value'],
    }
</script>