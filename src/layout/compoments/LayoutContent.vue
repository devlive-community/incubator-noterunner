<template>
  <div>
    <TinyLayout>
      <router-view></router-view>
    </TinyLayout>
  </div>
</template>

<script>
import {defineComponent} from "vue";
import {Layout as TinyLayout} from '@opentiny/vue'

export default defineComponent({
  name: 'LayoutContent',
  components: {TinyLayout}
});
</script>
