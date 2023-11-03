<template>
  <div>
    <TinyLayout :style="{backgroundColor: '#ffecec', height: height + 'px', padding: '10px 0px', textAlign: 'center'}">
    </TinyLayout>
  </div>
</template>

<script>
import {defineComponent} from "vue"
import {Layout as TinyLayout} from '@opentiny/vue'

export default defineComponent({
  name: 'LayoutAside',
  components: {TinyLayout},
  props: {
    height: {
      type: Number
    }
  }
});
</script>
