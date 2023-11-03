<template>
  <div>
    <TinyLayout>
      Aside
    </TinyLayout>
  </div>
</template>

<script>
import {defineComponent} from "vue";
import {Layout as TinyLayout} from '@opentiny/vue'

export default defineComponent({
  name: 'LayoutAside',
  components: {TinyLayout}
});
</script>
