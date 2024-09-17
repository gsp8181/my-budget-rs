<template>
    <div class="form-group">
        <label for="message-text" class="col-form-label">Day Taken:</label>
        <div class="input-group">
            <input type="number" placeholder="25" step="1" min="1" max="31" 
 class="form-control inputmodal-elem" name="day" v-bind:value="value"
      v-on:input="$emit('input', $event.target.value)" required></textarea>
        </div>
    </div>
</template>
<script>
//TODO: -1 do not display
    export default {
        name: 'day-popup',
        props: [ 'value'],
    }
</script>