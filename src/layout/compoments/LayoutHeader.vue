<template>
  <div>
    <TinyLayout>
      Header
    </TinyLayout>
  </div>
</template>

<script>
import {defineComponent} from "vue";
import {Layout as TinyLayout} from '@opentiny/vue'

export default defineComponent({
  name: 'LayoutHeader',
  components: {TinyLayout}
});
</script>
