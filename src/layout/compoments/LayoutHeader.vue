<template>
  <div>
    <TinyLayout :style="{backgroundColor: '#ecf8ff', height: height + 'px'}">
    </TinyLayout>
  </div>
</template>

<script>
import {defineComponent} from "vue"
import {Layout as TinyLayout} from '@opentiny/vue'

export default defineComponent({
  name: 'LayoutHeader',
  components: {TinyLayout},
  props: {
    height: {
      type: Number
    }
  }
});
</script>
