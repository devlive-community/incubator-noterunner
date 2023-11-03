<template>
  <div>
    <TinyLayout :style="{backgroundColor: '#FFFFFF', height: height + 'px'}">
      <TinyTabs v-model="activeTab"
                tab-style="card"
                :with-close="true">
        <TinyTabItem v-for="item in tabs"
                     :key="item.key"
                     :title="item.title"
                     :style="{margin: 0, padding: 0}"
                     :name="item.name">
          <MarkdownEditor v-if="item.editor === 'Markdown'"
                          :style="{height: (height - 75) + 'px'}"
                          :content="item.content"
                          @onChange="handlerChange($event, item)">
          </MarkdownEditor>
        </TinyTabItem>
      </TinyTabs>
    </TinyLayout>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue"
import {Layout as TinyLayout, TabItem as TinyTabItem, Tabs as TinyTabs} from '@opentiny/vue'
import MarkdownEditor from "../../components/MarkdownEditor.vue";
import {Note} from "../../model/note.ts";

export default defineComponent({
  name: 'LayoutContent',
  components: {MarkdownEditor, TinyLayout, TinyTabs, TinyTabItem},
  props: {
    height: {
      type: Number
    },
    note: {
      type: Object as () => Note
    }
  },
  data() {
    return {
      activeTab: String | undefined,
      tabs: []
    }
  },
  watch: {
    note() {
      if (this.note?.id.startsWith('custom_')) {
        this.tabs.push(this.note)
        this.activeTab = this.note.key
      }
    }
  },
  methods: {
    handlerChange(content: string, note: Note) {
      note.content = content
    }
  }
});
</script>
<style scoped>
.tiny-tabs__content {
  padding: 0;
}
</style>
