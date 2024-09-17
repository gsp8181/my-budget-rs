<template>
    <div class="form-group">
        <label for="message-text" class="col-form-label">Amount:</label>
        <div class="input-group">
            <div class="input-group-prepend">
                <div class="input-group-text">£</div>
            </div>
            <input type="number" placeholder="20.00" step="0.01" onchange="(function(el){el.value=parseFloat(el.value).toFixed(2);})(this)"
 class="form-control inputmodal-elem" name="amount" v-bind:value="value"
      v-on:input="$emit('input', $event.target.value)" required></textarea>
            <!--max and min todo-->
        </div>
    </div>
</template>
<script>
//TODO value is clickable if 0
//TODO: format
//TODO: test onchange
    export default {
        name: 'amount-popup',
        props: [ 'value'],
    }
</script>