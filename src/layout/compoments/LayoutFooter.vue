<template>
  <div>
    <TinyLayout>
      Footer
    </TinyLayout>
  </div>
</template>

<script>
import {defineComponent} from "vue"
import {Layout as TinyLayout} from '@opentiny/vue'

export default defineComponent({
  name: 'LayoutFooter',
  components: {TinyLayout}
});
</script>
