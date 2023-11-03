<template>
  <div>
    <TinyLayout :style="{backgroundColor: '#fffdec', height: height + 'px'}">
    </TinyLayout>
  </div>
</template>

<script>
import {defineComponent} from "vue"
import {Layout as TinyLayout} from '@opentiny/vue'

export default defineComponent({
  name: 'LayoutContent',
  components: {TinyLayout},
  props: {
    height: {
      type: Number
    }
  }
});
</script>
