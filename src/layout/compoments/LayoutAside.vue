<template>
  <div>
    <TinyLayout
        :style="{height: height + 'px', backgroundColor: 'rgb(239, 239, 242)', padding: '10px 0px', textAlign: 'center'}">
      <TinyPopover placement="bottom"
                   width="170"
                   trigger="hover"
                   popper-class="popperClass"
                   :visible-arrow="false">
        <div :style="{marginTop: '-20px'}">
          <TinyButton round
                      type="success"
                      size="medium"
                      :reset-time="0"
                      @click="handlerButtonClick('Markdown')">
            <div class="icon-container">
              <FontAwesomeIcon :icon="['fab', 'markdown']"></FontAwesomeIcon>
              <span class="icon-text">Markdown</span>
            </div>
          </TinyButton>
        </div>
        <template #reference>
          <TinyButton round
                      type="success"
                      size="medium">
            <IconPlusCircle
                style="font-size: 30px; margin-left: -25px; margin-top: -6px; margin-right: 15px;">
            </IconPlusCircle>
            新建笔记
          </TinyButton>
        </template>
      </TinyPopover>
      <TinyTree ref="tree"
                node-key="id"
                :data="data"
                :show-line="true"
                :style="{height: height + 'px', width: width + 'px', backgroundColor: 'rgb(239, 239, 242)', marginTop: '5px'}"
                @node-click="handlerNodeClick">
        <template #operation="{ node }">
          <div v-if="node.data.draft"
               class="operation-slot">
            <TinyTooltip effect="dark"
                         content="保存">
              <IconSave @click="handlerSave(node.data)"></IconSave>
            </TinyTooltip>
          </div>
          <TinyTooltip effect="dark"
                       content="删除">
            <IconDel @click="handlerDelete(node.data)"></IconDel>
          </TinyTooltip>
        </template>
      </TinyTree>
    </TinyLayout>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue"
import {
  Button as TinyButton,
  Divider as TinyDivider,
  Dropdown as TinyDropdown,
  DropdownItem as TinyDropdownItem,
  DropdownMenu as TinyDropdownMenu,
  Layout as TinyLayout,
  Notify,
  Popover as TinyPopover,
  Tooltip as TinyTooltip,
  Tree as TinyTree
} from '@opentiny/vue'
import {IconDel, IconPlusCircle, IconSave} from '@opentiny/vue-icon'
import {invoke} from '@tauri-apps/api/tauri'
import {Note} from "../../model/note.ts";
import {Response} from "../../model/response.ts";

export default defineComponent({
  name: 'LayoutAside',
  components: {
    TinyLayout,
    TinyDropdown,
    TinyDropdownMenu,
    TinyDropdownItem,
    TinyDivider,
    TinyTooltip,
    TinyButton,
    TinyPopover,
    TinyTree,
    IconSave: IconSave(),
    IconDel: IconDel(),
    IconPlusCircle: IconPlusCircle()
  },
  props: {
    height: {
      type: Number
    },
    width: {
      type: Number
    },
    node: {
      type: Object as () => Note
    }
  },
  data() {
    return {
      data: []
    }
  },
  created() {
    this.handlerInitialize()
  },
  methods: {
    handlerInitialize() {
      invoke('get_notes')
          .then(value => {
            const response = value as Response
            if (response.code === 200) {
              this.data = response.data
            } else {
              Notify({
                type: 'error',
                message: response.message,
                position: 'top',
                title: '错误',
                duration: 1000
              })
            }
          })
    },
    handlerButtonClick(type: string) {
      const key = Date.now().toString()
      const title = '无标题'
      const note: Note = {
        id: 'custom_' + key,
        key: key,
        title: title,
        label: title,
        name: key,
        editor: type,
        content: '',
        draft: true
      }
      const tree = this.$refs.tree as any
      tree.append(note)
      tree.setCurrentKey(note.id)
      this.$emit('onClick', note)
    },
    handlerSave(note: Note) {
      if (!note.modify) {
        note.id = 0
      }
      const cmd = note.modify ? 'update_note' : 'create_note'
      invoke(cmd, {note: note})
          .then(value => {
            const response = value as Response
            if (response.code === 200 && response.data) {
              const message = note.modify ? `保存 [ ${note.title} ] 成功` : `新建 [ ${note.title} ] 成功`
              Notify({
                type: 'success',
                message: message,
                position: 'top',
                title: '提示',
                duration: 1000
              })
              note.draft = false
              note.saved = true
              this.$emit('onClick', note)
              this.handlerInitialize()
            } else {
              Notify({
                type: 'error',
                message: response.message,
                position: 'top',
                title: '错误',
                duration: 1000
              })
            }
          })
    },
    handlerNodeClick(data: Note) {
      this.$emit('onClick', data)
    },
    handlerDelete(note: Note) {
      if (note.id.toString().startsWith('custom_')) {
        const tree = this.$refs.tree as any
        tree.remove(note.id)
      } else {
        invoke('delete_note', {id: note.id})
            .then(value => {
              const response = value as Response
              if (response.code === 200 && response.data) {
                Notify({
                  type: 'success',
                  message: `删除 [ ${note.title} ] 成功`,
                  position: 'top',
                  title: '提示',
                  duration: 1000
                })
                this.$emit('onDelete', note)
                this.handlerInitialize()
              } else {
                Notify({
                  type: 'error',
                  message: response.message,
                  position: 'top',
                  title: '错误',
                  duration: 1000
                })
              }
            })
      }
    }
  },
  watch: {
    node() {
      const tree = this.$refs.tree as any
      if (this.node) {
        tree.setCurrentKey(this.node.id)
      } else {
        tree.setCurrentKey(undefined)
      }
    },
    'node.modify': {
      handler() {
        if (this.node) {
          this.node.draft = true
        }
      }
    }
  }
});
</script>

<style>
.popperClass {
  background-color: transparent !important;
  -webkit-box-shadow: none !important;
  box-shadow: none !important;
  border: 0px solid transparent !important;
}

.icon-container {
  display: flex;
  align-items: center;
}

.icon-container svg {
  margin-right: 0.5em;
  font-size: 20px;
  margin-left: -1em;
}
</style>
