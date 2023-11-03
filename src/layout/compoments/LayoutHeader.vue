<template>
  <div>
    <TinyLayout :style="{backgroundColor: '#FFFFFF', height: height + 'px'}">
      <TinyTabs v-model="activeTab"
                tab-style="card"
                :with-close="true">
        <TinyTabItem v-for="item in tabs"
                     :key="item.key"
                     :title="item.title"
                     :name="item.name">
          <tiny-button type="success">按钮</tiny-button>
          <MarkdownEditor v-if="type === 'Markdown'"
                          :style="{height: (height - 55) + 'px', width: '100%'}"
                          :content="item.content">
          </MarkdownEditor>
        </TinyTabItem>
      </TinyTabs>
    </TinyLayout>
  </div>
</template>

<script>
import {defineComponent} from "vue"
import {Layout as TinyLayout, TabItem as TinyTabItem, Tabs as TinyTabs} from '@opentiny/vue'
import MarkdownEditor from "../../components/MarkdownEditor.vue";

export default defineComponent({
  name: 'LayoutHeader',
  components: {MarkdownEditor, TinyLayout, TinyTabs, TinyTabItem},
  props: {
    height: {
      type: Number
    },
    type: {
      type: String
    },
    command: {
      type: String
    }
  },
  data() {
    return {
      activeTab: 'first',
      tabs: []
    }
  },
  watch: {
    command() {
      if (this.command === 'created') {
        const key = Date.now().toString()
        this.tabs.push({
          key: key,
          title: '新笔记',
          name: key,
          content: ''
        })
        this.activeTab = key
      }
    }
  },
  methods: {}
});
</script>
<style scoped>
.tiny-tabs__content {
  padding: 0;
}
</style>
