<template>
  <div>
    <MdEditor v-model="localContent"
              :style="{height: height + 'px'}"
              :toolbarsExclude="['fullscreen', 'github', 'htmlPreview', 'save']"
              :editorId="key">
    </MdEditor>
  </div>
</template>

<script>
import {defineComponent} from 'vue'
import {MdEditor} from 'md-editor-v3'
import 'md-editor-v3/lib/style.css'

export default defineComponent({
  name: 'MarkdownEditor',
  components: {MdEditor},
  props: {
    content: {
      type: String
    },
    key: {
      type: String | Number
    },
    height: {
      type: Number
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
