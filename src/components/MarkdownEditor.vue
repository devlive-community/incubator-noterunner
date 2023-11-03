<template>
  <div>
    <VMarkdownEditor v-model="localContent">
    </VMarkdownEditor>
  </div>
</template>

<script>
import {defineComponent} from 'vue'
import {VMarkdownEditor} from 'vue3-markdown'
import 'vue3-markdown/dist/style.css'

export default defineComponent({
  name: 'MarkdownEditor',
  components: {VMarkdownEditor},
  props: {
    content: {
      type: String
    }
  },
  data() {
    return {
      localContent: this.content
    }
  },
  watch: {
    localContent() {
      this.$emit('onChange', this.localContent)
    }
  }
});
</script>
