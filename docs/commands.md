# Command reference

<!-- Generated from the engine's catalog: `cargo run -p cc_cli -- reference > docs/commands.md`. Do not edit by hand. -->

Every command runs the same way from the desktop app, `ccad run` / `ccad script` and MCP (see [the automation guide](automation.md)). Lengths are in meters, angles in degrees, points `[x, y]` in plan (in a view's drawing coordinates for dimensions). *id or name* parameters accept a numeric id or the name of a level, type, view or material. Kinds: **query** reads the model, **edit** changes it (one undo step, refused if it would leave the model invalid), **file** works with files or replaces the project.

## Contents

- **Project and files**: [`project_new`](#project_new), [`project_open`](#project_open), [`project_save`](#project_save), [`project_info`](#project_info), [`project_info_set`](#project_info_set), [`site_get`](#site_get), [`site_set`](#site_set), [`site_context_import`](#site_context_import), [`undo`](#undo), [`redo`](#redo), [`units_get`](#units_get), [`units_set`](#units_set), [`language_set`](#language_set), [`ifc_import`](#ifc_import), [`skp_import`](#skp_import), [`skp_component_load`](#skp_component_load), [`obj_import`](#obj_import), [`gltf_import`](#gltf_import), [`ifc_export`](#ifc_export), [`sheet_export`](#sheet_export), [`view_export`](#view_export), [`view_render`](#view_render), [`schedule_export_csv`](#schedule_export_csv), [`component_load`](#component_load), [`component_library_list`](#component_library_list), [`component_save`](#component_save), [`cad_import`](#cad_import), [`image_import`](#image_import), [`cad_layers`](#cad_layers), [`cad_layers_set`](#cad_layers_set), [`cad_explode`](#cad_explode)
- **Queries**: [`categories_list`](#categories_list), [`levels_list`](#levels_list), [`types_list`](#types_list), [`type_get`](#type_get), [`elements_list`](#elements_list), [`element_get`](#element_get), [`groups_list`](#groups_list), [`measure`](#measure), [`views_list`](#views_list), [`view_get`](#view_get), [`materials_list`](#materials_list), [`material_get`](#material_get), [`schedule_table`](#schedule_table)
- **Creating elements**: [`wall_create`](#wall_create), [`curtain_type_create`](#curtain_type_create), [`curtain_get`](#curtain_get), [`curtain_grid_set`](#curtain_grid_set), [`curtain_panel_set`](#curtain_panel_set), [`door_create`](#door_create), [`door_type_create`](#door_type_create), [`window_create`](#window_create), [`niche_type_create`](#niche_type_create), [`niche_create`](#niche_create), [`floor_create`](#floor_create), [`floor_opening_add`](#floor_opening_add), [`floor_opening_remove`](#floor_opening_remove), [`ceiling_create`](#ceiling_create), [`roof_create`](#roof_create), [`roof_edge_set`](#roof_edge_set), [`roof_opening_remove`](#roof_opening_remove), [`room_create`](#room_create), [`grid_create`](#grid_create), [`column_create`](#column_create), [`beam_create`](#beam_create), [`columns_at_grids`](#columns_at_grids), [`beams_on_grids`](#beams_on_grids), [`beams_between_columns`](#beams_between_columns), [`grid_frame_create`](#grid_frame_create), [`slab_join_supports`](#slab_join_supports), [`structure_number`](#structure_number), [`brace_create`](#brace_create), [`braces_between_columns`](#braces_between_columns), [`truss_create`](#truss_create), [`bracing_create`](#bracing_create), [`rebar_create`](#rebar_create), [`rebar_typical`](#rebar_typical), [`rebar_copy`](#rebar_copy), [`wall_rebar_create`](#wall_rebar_create), [`slab_rebar_create`](#slab_rebar_create), [`strip_rebar_create`](#strip_rebar_create), [`bar_marks_list`](#bar_marks_list), [`bar_marks_update`](#bar_marks_update), [`bar_marks_renumber`](#bar_marks_renumber), [`bar_marking_set`](#bar_marking_set), [`bar_length_check`](#bar_length_check), [`bending_schedule_create`](#bending_schedule_create), [`bar_tag_create`](#bar_tag_create), [`connections_create`](#connections_create), [`analytical_model`](#analytical_model), [`analytical_export`](#analytical_export), [`footing_create`](#footing_create), [`strip_footing_create`](#strip_footing_create), [`foundation_slab_create`](#foundation_slab_create), [`footings_under_columns`](#footings_under_columns), [`strip_footings_under_walls`](#strip_footings_under_walls), [`pile_type_create`](#pile_type_create), [`pile_create`](#pile_create), [`pile_cap_type_create`](#pile_cap_type_create), [`pile_cap_create`](#pile_cap_create), [`pile_caps_under_columns`](#pile_caps_under_columns), [`tie_beams_create`](#tie_beams_create), [`stair_create`](#stair_create), [`stair_check`](#stair_check), [`stair_railings_add`](#stair_railings_add), [`railing_host_set`](#railing_host_set), [`ramp_create`](#ramp_create), [`ramp_check`](#ramp_check), [`roof_check`](#roof_check), [`mass_create`](#mass_create), [`sketch_get`](#sketch_get), [`sketch_set`](#sketch_set), [`mass_floors_set`](#mass_floors_set), [`mass_floors`](#mass_floors), [`mass_type_create`](#mass_type_create), [`sweep_create`](#sweep_create), [`sweep_type_create`](#sweep_type_create), [`sweep_profile_set`](#sweep_profile_set), [`shaft_create`](#shaft_create), [`shaft_type_create`](#shaft_type_create), [`topography_create`](#topography_create), [`topography_points_set`](#topography_points_set), [`topography_type_create`](#topography_type_create), [`pad_create`](#pad_create), [`pad_type_create`](#pad_type_create), [`mass_face_move`](#mass_face_move), [`mass_walls_create`](#mass_walls_create), [`mass_slabs_create`](#mass_slabs_create), [`mass_roof_create`](#mass_roof_create), [`ramp_type_create`](#ramp_type_create), [`railing_create`](#railing_create), [`component_place`](#component_place)
- **Building services**: [`pipe_create`](#pipe_create), [`conduit_create`](#conduit_create), [`cable_tray_create`](#cable_tray_create), [`cable_tray_type_create`](#cable_tray_type_create), [`valve_place`](#valve_place), [`valve_type_create`](#valve_type_create), [`device_type_create`](#device_type_create), [`lps_conductor_create`](#lps_conductor_create), [`lps_conductor_type_create`](#lps_conductor_type_create), [`earth_ring_create`](#earth_ring_create), [`air_terminal_place`](#air_terminal_place), [`earth_electrode_place`](#earth_electrode_place), [`lps_check`](#lps_check), [`rack_place`](#rack_place), [`data_outlet_place`](#data_outlet_place), [`cabling_list`](#cabling_list), [`systems_list`](#systems_list), [`system_create`](#system_create), [`system_set`](#system_set), [`system_delete`](#system_delete), [`system_assign`](#system_assign), [`circuit_loads`](#circuit_loads), [`mep_fittings`](#mep_fittings), [`mep_connect`](#mep_connect), [`mep_sizing`](#mep_sizing), [`mep_size_apply`](#mep_size_apply)
- **Modifying elements**: [`elements_delete`](#elements_delete), [`elements_move`](#elements_move), [`elements_copy`](#elements_copy), [`elements_rotate`](#elements_rotate), [`elements_mirror`](#elements_mirror), [`elements_pin`](#elements_pin), [`element_flip`](#element_flip), [`elements_turn`](#elements_turn), [`elements_export`](#elements_export), [`elements_paste`](#elements_paste), [`wall_join`](#wall_join), [`wall_attach`](#wall_attach), [`geometry_join`](#geometry_join), [`geometry_unjoin`](#geometry_unjoin), [`group_create`](#group_create), [`group_ungroup`](#group_ungroup), [`trim_corner`](#trim_corner), [`trim_extend`](#trim_extend), [`element_split`](#element_split), [`element_offset`](#element_offset), [`element_align`](#element_align), [`element_move_end`](#element_move_end), [`grids_align_ends`](#grids_align_ends), [`element_set`](#element_set), [`elements_set`](#elements_set), [`elements_set_type`](#elements_set_type)
- **Dimensions and text**: [`dimension_create`](#dimension_create), [`tag_move`](#tag_move), [`dimension_chain_create`](#dimension_chain_create), [`dimension_ordinate_create`](#dimension_ordinate_create), [`dimension_set_value`](#dimension_set_value), [`text_create`](#text_create), [`detail_line_create`](#detail_line_create), [`detail_component_create`](#detail_component_create), [`detail_component_place`](#detail_component_place), [`browser_schemes_list`](#browser_schemes_list), [`browser_scheme_create`](#browser_scheme_create), [`browser_scheme_set`](#browser_scheme_set), [`browser_scheme_delete`](#browser_scheme_delete), [`browser_scheme_use`](#browser_scheme_use), [`browser_tree`](#browser_tree), [`line_styles_list`](#line_styles_list), [`line_style_create`](#line_style_create), [`line_style_set`](#line_style_set), [`line_style_delete`](#line_style_delete), [`filled_region_create`](#filled_region_create), [`masking_region_create`](#masking_region_create), [`filled_region_type_create`](#filled_region_type_create), [`level_mark_create`](#level_mark_create), [`level_mark_type_create`](#level_mark_type_create), [`text_add_leader`](#text_add_leader), [`text_remove_leader`](#text_remove_leader)
- **Types, components, materials**: [`type_set`](#type_set), [`type_duplicate`](#type_duplicate), [`type_delete`](#type_delete), [`component_create`](#component_create), [`component_get`](#component_get), [`component_param_add`](#component_param_add), [`component_param_set`](#component_param_set), [`component_param_delete`](#component_param_delete), [`component_ref_plane_add`](#component_ref_plane_add), [`component_ref_plane_set`](#component_ref_plane_set), [`component_ref_plane_delete`](#component_ref_plane_delete), [`component_ref_line_add`](#component_ref_line_add), [`component_ref_line_delete`](#component_ref_line_delete), [`component_dimension_add`](#component_dimension_add), [`component_dimension_set`](#component_dimension_set), [`component_dimension_delete`](#component_dimension_delete), [`component_equal_add`](#component_equal_add), [`component_equal_delete`](#component_equal_delete), [`component_piece_add`](#component_piece_add), [`component_piece_set`](#component_piece_set), [`component_piece_delete`](#component_piece_delete), [`component_connector_add`](#component_connector_add), [`component_connector_set`](#component_connector_set), [`component_connector_delete`](#component_connector_delete), [`component_script_set`](#component_script_set), [`component_params_get`](#component_params_get), [`component_params_set`](#component_params_set), [`component_hotspot_move`](#component_hotspot_move), [`material_create`](#material_create), [`material_duplicate`](#material_duplicate), [`material_rename`](#material_rename), [`material_set`](#material_set), [`material_delete`](#material_delete), [`material_texture_set`](#material_texture_set), [`material_texture_clear`](#material_texture_clear), [`fill_patterns_list`](#fill_patterns_list), [`fill_pattern_get`](#fill_pattern_get), [`fill_pattern_create`](#fill_pattern_create), [`fill_pattern_set`](#fill_pattern_set), [`fill_pattern_duplicate`](#fill_pattern_duplicate), [`fill_pattern_delete`](#fill_pattern_delete), [`fill_pattern_import`](#fill_pattern_import), [`fill_pattern_export`](#fill_pattern_export), [`profiles_list`](#profiles_list), [`profile_get`](#profile_get), [`profile_create`](#profile_create), [`profile_set`](#profile_set), [`profile_duplicate`](#profile_duplicate), [`profile_delete`](#profile_delete), [`profile_catalog`](#profile_catalog), [`profile_import`](#profile_import), [`profile_type_create`](#profile_type_create), [`structural_type_for`](#structural_type_for), [`steel_building_create`](#steel_building_create), [`wall_framing_create`](#wall_framing_create), [`paint`](#paint), [`paint_remove`](#paint_remove), [`paints_list`](#paints_list)
- **Levels and views**: [`level_create`](#level_create), [`level_set`](#level_set), [`level_delete`](#level_delete), [`view_create`](#view_create), [`view_set`](#view_set), [`schedule_get`](#schedule_get), [`schedule_set`](#schedule_set), [`view_duplicate`](#view_duplicate), [`view_delete`](#view_delete), [`callout_create`](#callout_create), [`callout_remove`](#callout_remove), [`callouts_list`](#callouts_list), [`sun_position`](#sun_position), [`sun_frames`](#sun_frames), [`sun_presets_list`](#sun_presets_list), [`sun_preset_save`](#sun_preset_save), [`sun_preset_rename`](#sun_preset_rename), [`sun_preset_delete`](#sun_preset_delete), [`sheet_place_view`](#sheet_place_view), [`sheet_remove_viewport`](#sheet_remove_viewport), [`sheet_set`](#sheet_set), [`sheet_fields`](#sheet_fields), [`title_block_create`](#title_block_create), [`component_sheet_piece_add`](#component_sheet_piece_add), [`component_sheet_piece_set`](#component_sheet_piece_set), [`component_sheet_piece_delete`](#component_sheet_piece_delete)
- **View graphics**: [`view_graphics_get`](#view_graphics_get), [`view_graphics_set`](#view_graphics_set), [`element_override_set`](#element_override_set), [`element_hide_in_view`](#element_hide_in_view), [`filters_list`](#filters_list), [`filter_create`](#filter_create), [`filter_set`](#filter_set), [`filter_delete`](#filter_delete), [`filter_elements`](#filter_elements), [`view_filter_add`](#view_filter_add), [`view_filter_set`](#view_filter_set), [`view_filter_remove`](#view_filter_remove), [`view_filter_move`](#view_filter_move), [`templates_list`](#templates_list), [`template_create`](#template_create), [`template_set`](#template_set), [`template_delete`](#template_delete), [`view_template_apply`](#view_template_apply), [`view_template_assign`](#view_template_assign), [`view_temporary`](#view_temporary), [`view_temporary_apply`](#view_temporary_apply)
- **Phases**: [`phases_list`](#phases_list), [`phase_create`](#phase_create), [`phase_set`](#phase_set), [`phase_delete`](#phase_delete), [`elements_phase_set`](#elements_phase_set), [`elements_demolish`](#elements_demolish), [`phase_creation_set`](#phase_creation_set), [`phase_filters_list`](#phase_filters_list), [`phase_filter_create`](#phase_filter_create), [`phase_filter_set`](#phase_filter_set), [`phase_filter_delete`](#phase_filter_delete), [`phase_graphics_get`](#phase_graphics_get), [`phase_graphics_set`](#phase_graphics_set)
- **Design options**: [`options_list`](#options_list), [`option_set_create`](#option_set_create), [`option_set_rename`](#option_set_rename), [`option_set_delete`](#option_set_delete), [`option_create`](#option_create), [`option_set`](#option_set), [`option_delete`](#option_delete), [`option_accept_primary`](#option_accept_primary), [`elements_option_set`](#elements_option_set), [`view_options_set`](#view_options_set), [`option_active_set`](#option_active_set)
- **Visual programming**: [`graph_run`](#graph_run), [`graph_check`](#graph_check), [`graph_nodes`](#graph_nodes), [`graph_elements`](#graph_elements), [`graph_release`](#graph_release)

## Project and files

### `project_new`

*file* — Start a new project from a template (unsaved changes are discarded).

| Parameter | Type | | Description |
|---|---|---|---|
| `template` | string | optional | The template: `metric` (the default; meters, metric sizes) or `imperial` (feet and inches, round imperial sizes). |

### `project_open`

*file* — Open a ConstructCraft project (.ccad), replacing the session's project. Damaged files are repaired and the number of repairs reported.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the .ccad file. |

### `project_save`

*file* — Save the project to its file, or to `path` (which becomes its file).

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | optional | Where to save; required when the project has no file yet. |

### `project_info`

*query* — Project information, file, unsaved state and counts.

No parameters.

### `project_info_set`

*edit* — Change the project information (title block fields).

| Parameter | Type | | Description |
|---|---|---|---|
| `address` | string | optional | Project address. |
| `author` | string | optional | Author of the drawings. |
| `client` | string | optional | Client name. |
| `fields` | object | optional | Project fields by name: a text sets the field (adding it; a new name holds no braces and at most 192 bytes), `null` removes it. |
| `name` | string | optional | Project name. |
| `number` | string | optional | Project number. |

### `site_get`

*query* — The project's site location: latitude and longitude of the project origin (degrees, WGS 84), its elevation above sea level (meters), the angle from project north to true north (degrees counter-clockwise), the place name, the parts of the postal address, and local time's offset from UTC in hours (`utc_offset`, null when not set, and `utc_offset_used`: the one set, else the longitude's nominal time zone, which the sun uses); `site` is null when no location is set. `address` is the project address line.

No parameters.

### `site_set`

*edit* — Set the project's site location, in one undo step: the latitude and longitude of the project origin (both needed when the project has no location yet), its elevation above sea level, true north and the place and address found by a map search, and the offset from UTC. Fields left out keep their values. When address parts are given the project address (title blocks, IFC) is rewritten from them unless `project_address` is false. `clear` removes the location.

| Parameter | Type | | Description |
|---|---|---|---|
| `address` | object | optional | Address parts by key (`street`, `number`, `district`, `city`, `region`, `postcode`, `country`); others are refused. |
| `clear` | boolean | optional | Remove the site location (nothing else may be given). |
| `elevation` | number | optional | Height of the project's elevation 0 above sea level, meters (-500 to 9000). |
| `latitude` | number | optional | Latitude of the project origin, degrees (WGS 84, -90 to 90, north positive). |
| `longitude` | number | optional | Longitude of the project origin, degrees (WGS 84, -180 to 180, east positive). |
| `place` | string | optional | The place as the map search named it. |
| `project_address` | boolean | optional | Rewrite the project address from the address parts (default true when `address` is given). |
| `true_north` | number | optional | Angle from project north (+Y) to true north, degrees counter-clockwise. |
| `utc_offset` | number | optional | Local time's offset from UTC, hours (-12 to 14; fractions allowed), or `null` for the longitude's nominal time zone. |

### `site_context_import`

*edit* — Bring OpenStreetMap data around the site into a floor plan, in one undo step: a site drawing (an imported drawing in meters, pinned, under the model: layers for buildings, streets, paths, railways, water, green areas, trees and street names) and, unless `masses` is false, a context mass per building footprint (height from the map's `height` or `building:levels`, else `default_levels` storeys; no mass floors) of the "Context" mass type. `data` is the JSON an Overpass query answers (`[out:json]`, `out geom` or ways with their nodes; the program builds the query for the square it shows); points are placed with the site location, which must be set. Map data © OpenStreetMap contributors (ODbL).

| Parameter | Type | | Description |
|---|---|---|---|
| `data` | string | optional | Overpass JSON text (or `path`). |
| `default_levels` | number | optional | Storeys of a building whose height the map does not give (default 2). |
| `extent` | number | optional | Half the side of the square kept around the project origin, meters (default: everything given). |
| `level_height` | number | optional | Height of a storey, meters, for `building:levels` (default 3). |
| `masses` | boolean | optional | Make context masses from the building footprints (default true). |
| `path` | string | optional | Path of a file holding Overpass JSON (or `data`). |
| `text_height` | number | optional | Height of street names, meters (default 2). |
| `view` | id or name | optional | Floor plan id or name (default: the first floor plan of the lowest level). |

### `undo`

*edit* — Undo the last edit.

No parameters.

### `redo`

*edit* — Redo the last undone edit.

No parameters.

### `units_get`

*query* — The project units (length, area, volume, angle, slope, currency, number format).

No parameters.

### `units_set`

*edit* — Change the project units: `values` is merged into the object `units_get` returns (only the fields given change), after `preset` when given. Length units are `Millimeter`, `Centimeter`, `Meter`, `Foot`, `Inch`, `FeetInches` (`12'-6 1/2"`) and `FractionalInch`; for the last two, `decimals` is the precision, 1/2^decimals of an inch.

| Parameter | Type | | Description |
|---|---|---|---|
| `preset` | string | optional | Start from `metric` (the defaults) or `imperial` (feet and inches to 1/16", square and cubic feet). |
| `values` | JSON | optional | Fields to change, shaped like the result of `units_get`. |

### `language_set`

*edit* — Language of default names, messages and sheet labels (`en`, `pt-BR`, `es`, `fr`, `de`, `it`, `lo`, `pl`, `uk-UA`, `zh-TW`, `zh-CN` or `ja`).

| Parameter | Type | | Description |
|---|---|---|---|
| `language` | string | required | Language code: `en`, `pt-BR`, `es`, `fr`, `de`, `it`, `lo`, `pl`, `uk-UA`, `zh-TW`, `zh-CN` or `ja`. |

### `ifc_import`

*file* — Import an IFC file (IFC2X3 or IFC4), replacing the session's project. The result has no file yet; save it with `project_save` and a path.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the .ifc file. |

### `skp_import`

*edit* — Import a SketchUp model (.skp) into the project. Each placed component or group becomes a component element on `level`, of a component type holding its faces as fixed mesh pieces (one per material): placements that differ only by a move or a plan rotation share a type, anything else in their transform (tilt, scale, mirror) is built into the type. Faces outside any component make one more element at the file's origin. Missing materials are added by name, color, opacity and texture; hidden objects and those on hidden layers are left out.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the .skp file. |
| `level` | id or name | optional | Level id or name (default: the lowest level); the file's ground is at its elevation. |
| `position` | [x, y] | optional | Where the file's origin goes, in plan (default [0, 0]). |
| `rotation` | number | optional | Rotation of the whole model in plan, degrees counter-clockwise (default 0). |

### `skp_component_load`

*edit* — Load a SketchUp model (.skp) as one component type, named after the file, to place like any component: its faces become fixed mesh pieces (one per material) with the file's origin as the insertion point. Missing materials are added by name and color.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the .skp file. |

### `obj_import`

*edit* — Import a Wavefront OBJ model (.obj) into the project. Each object (or, in a file without objects, each group) becomes a component element on `level`, of a component type holding its faces as fixed mesh pieces, one per material, inserted at the center of its footprint and the bottom of its extent. Materials come from the `.mtl` libraries next to the file: color (`Kd`), opacity (`d`, `Tr`) and texture (`map_Kd`, the picture next to the file, repeating as the faces' `vt` coordinates say); missing materials are added by name. Faces not flat are split into triangles; lines, points and normals are ignored.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the .obj file. |
| `level` | id or name | optional | Level id or name (default: the lowest level); the file's zero height is at its elevation. |
| `merge` | boolean | optional | Make the whole file one component element, inserted at the file's origin. |
| `position` | [x, y] | optional | Where the file's origin goes, in plan (default [0, 0]). |
| `rotation` | number | optional | Rotation of the whole model in plan, degrees counter-clockwise (default 0). |
| `units` | string | optional | Units of the file's coordinates: "m" (default), "cm", "mm", "in" or "ft". |
| `z_up` | boolean | optional | The file's Z axis points up; by default Y does (the usual OBJ convention). |

### `gltf_import`

*edit* — Import a glTF 2.0 model (.glb, or .gltf with its buffers embedded or in files next to it) into the project. Each node of the scene's root, with everything below it, becomes a component element on `level`, of a component type holding its triangles as fixed mesh pieces, one per material, inserted at the center of its footprint and the bottom of its extent. glTF is in meters with Y up. Materials take their base color times its texture's average, the texture itself (repeating as the texture coordinates say) and, when blended, their opacity; missing materials are added by name. Lines and points are ignored, and compressed meshes (Draco, meshopt) are refused.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the .glb or .gltf file. |
| `level` | id or name | optional | Level id or name (default: the lowest level); the file's zero height is at its elevation. |
| `merge` | boolean | optional | Make the whole scene one component element, inserted at the file's origin. |
| `position` | [x, y] | optional | Where the file's origin goes, in plan (default [0, 0]). |
| `rotation` | number | optional | Rotation of the whole model in plan, degrees counter-clockwise (default 0). |

### `ifc_export`

*file* — Export the project as IFC4.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the .ifc file to write. |

### `sheet_export`

*file* — Export a sheet as vector PDF or SVG, or as DXF in paper millimeters (by `format`, else by the file extension).

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the file to write (.pdf, .svg or .dxf). |
| `sheet` | id or name | required | Sheet id or name. |
| `format` | string | optional | `pdf`, `svg` or `dxf`. |

### `view_export`

*file* — Export any view with a drawing (plan, elevation, section, 3D view as a hidden-line axonometric, legend, drafting view, diagram or sheet) as a PNG image, as vector SVG or PDF, or as DXF for other CAD programs (by `format`, else by the file extension). The view is drawn at its scale as printed, without temporary hiding, on a page that fits it; sheets keep their paper size. A DXF holds the view at full size in millimeters (a sheet in paper millimeters), each element on the layer of its category (detail lines, and regions with a boundary, on the layer named after their line style; region fills as `SOLID` triangles, their patterns as lines), colors as the nearest color index and line patterns as line types.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the file to write (.png, .svg, .pdf or .dxf). |
| `view` | id or name | required | View id or name. |
| `dpi` | number | optional | Resolution of a PNG, in dots per inch (default 150). |
| `format` | string | optional | `png`, `svg`, `pdf` or `dxf`. |

### `view_render`

*file* — Render a 3D view photorealistically (path tracing on the processor, no GPU) and write the image: `.png`, `.jpg`/`.jpeg` exposed, tone mapped and sRGB encoded, `.exr` and `.hdr` in linear light. The view's render settings are used (the defaults when it has none), patched by `settings` for this render only; its camera is the one kept in the view, fitted to the model when it was never moved, as the 3D view does. The view is rendered as printed, without temporary hiding. Changes nothing in the model. Returns the path, the image size, the samples per pixel, the seconds spent and the triangles rendered.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the image to write (.png, .jpg, .jpeg, .exr or .hdr). |
| `view` | id or name | required | 3D view id or name. |
| `region` | [id, …] | optional | Render only this rectangle of the image, `[x, y, width, height]` in pixels from its top left corner (inside the image); the rest stays black. |
| `settings` | JSON | optional | Render settings for this render only, not kept in the view: the fields to change over the view's settings (or the defaults when it has none), shaped and bounded like `render` in `view_set`. |

### `schedule_export_csv`

*file* — Export a schedule view as CSV (UTF-8; semicolons when the decimal comma is used).

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the .csv file to write. |
| `view` | id or name | required | Schedule view id or name. |

### `component_load`

*edit* — Load a component file (.ccomp): its types and missing materials join the project.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the .ccomp file. |

### `component_library_list`

*file* — List the component library: the component files (.ccomp) in `folders` and their subfolders, each with its component, kind, category (doors and windows have their own), types, parameters and connectors; `search` keeps those whose component, type names or descriptions contain it. Files that do not load are listed with the reason.

| Parameter | Type | | Description |
|---|---|---|---|
| `folders` | [string, …] | required | Folders to look in. |
| `search` | string | optional | Text to look for (any case). |

### `component_save`

*file* — Save a component (all its types and the materials they use) to a .ccomp file.

| Parameter | Type | | Description |
|---|---|---|---|
| `component` | string | required | Component name (as in a component type's `params.component`). |
| `path` | string | required | Path of the .ccomp file to write. |

### `cad_import`

*edit* — Import a DWG or DXF file into a 2D view (plan, ceiling plan, elevation, section, legend or drafting view) as a drawing to trace over, in one undo step. DXF (ASCII or binary, any version) is read entirely by ConstructCraft; DWG (R12 to 2018 and later) through the acadrust library. Blocks are inserted, curves kept as arcs, texts kept; entities that are not drawn (3D solids, images, infinite lines…) are counted in `skipped`. The element starts pinned, with the layers that are off or frozen in the file hidden, and in the view's level. It is drawn in the view only, under the model, and is left out of 3D views and IFC export.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the .dwg or .dxf file. |
| `view` | id or name | required | View id or name. |
| `align` | string | optional | `origin` (the drawing's origin at `position`), `center`, or `auto`: the origin, unless that leaves the drawing more than 1 km from what the view shows (a drawing in survey coordinates), then the center. |
| `blocks` | boolean | optional | Bring the file's blocks in as detail components (DWG and DXF): each block inserted by a move, turn, uniform scale or mirror becomes a detail component type named after it (its nested blocks as nested pieces), and each such insert a detail component placed in the view; everything else stays in the imported drawing (default false: blocks are flattened into it). |
| `color_mode` | string | optional | `grey` (default), `layers` (the file's colors) or `black`. |
| `halftone` | boolean | optional | Draw it faded (default false). |
| `position` | [x, y] | optional | Where the drawing goes, view coordinates in meters: its origin (`align` "origin", the default; [0, 0] if not given) or the center of its extents (`align` "center"; the center of what the view shows if not given). |
| `rotation` | number | optional | Rotation in degrees counter-clockwise. |
| `scale` | number | optional | Extra scale on top of the units (default 1). |
| `units` | string | optional | Drawing units, overriding the file's: mm, cm, dm, m, km, in, ft, yd or mi. Unitless files are read as meters when small (up to 1500 units across), else as millimeters. |

### `image_import`

*edit* — Import a picture (PNG, JPEG, GIF, BMP or WebP) or an SVG drawing into a 2D view (plan, ceiling plan, elevation, section, legend or drafting view), in one undo step, as an imported drawing under the model: placed, scaled, rotated, pinned, hidden and printed like an imported DWG or DXF. A picture is as large as its resolution makes it on paper (96 dpi when the file declares none), times `scale`, unless `width` sets its width in the view; the file's bytes are kept in the project. An SVG keeps its paths (curves as polylines), fills and embedded pictures in their own colors; its texts are not read.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the picture or .svg file. |
| `view` | id or name | required | View id or name. |
| `align` | string | optional | `center` (default) or `origin`. |
| `halftone` | boolean | optional | Draw it faded (default false). |
| `position` | [x, y] | optional | Where it goes, view coordinates in meters: the center of the picture (`align` "center", the default; the center of what the view shows if not given) or its bottom-left corner for pictures and the page's for SVG (`align` "origin"; [0, 0] if not given). |
| `rotation` | number | optional | Rotation in degrees counter-clockwise. |
| `scale` | number | optional | Scale on top of its size on paper (default 1; 100 for a plan drawn at 1:100). |
| `width` | number | optional | Width in the view, meters (overrides `scale`). |

### `cad_layers`

*query* — The layers of an imported drawing: name, color, whether the view shows it, whether it was on in the file, and how many primitives it holds.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Imported drawing element id. |

### `cad_layers_set`

*edit* — Show or hide layers of an imported drawing (by name; all of them when `layers` is empty).

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Imported drawing element id. |
| `visible` | boolean | required | Show (true) or hide (false) them. |
| `layers` | [string, …] | optional | Layer names (case-insensitive); empty for every layer. |

### `cad_explode`

*edit* — Explode an imported DWG, DXF or SVG drawing into native elements of its view, in one undo step: its lines, polylines, arcs and circles become detail lines (one element per segment, bulged segments as arcs; ellipses and splines are stored as polylines and come out as their segments), its fills filled regions (no boundary, a solid filled region type of the fill's color, reused when the project has one), and its texts text notes (same place, height, rotation and alignment; a text type of that size on paper at the view's scale, reused when one exists). Each layer becomes a project line style of its name and color (light colors darkened for paper), reused when a style of that name exists. Layers hidden in the element (which starts with the layers off or frozen in the file) are left out; the placement (units, scale, position, rotation, mirror) is applied. Pictures stay in an imported drawing in the same view with the same placement; without them the element is removed. The color mode and halftone are not carried over. Refused above 100,000 elements.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Imported drawing element id. |

## Queries

### `categories_list`

*query* — Element categories with their display names and how many elements and types each has.

No parameters.

### `levels_list`

*query* — Levels from the lowest, with their elevations (meters).

No parameters.

### `types_list`

*query* — Types, optionally of one category.

| Parameter | Type | | Description |
|---|---|---|---|
| `category` | string | optional | Category name (`wall`, `door`, `component`…). |

### `type_get`

*query* — A type with all its parameters (`params`, editable with `type_set`) and identity data (`info`). A title block's pictures show as `format`, `width`, `height` (pixels) and `bytes`, not their bytes; `component_sheet_piece_set` replaces them.

| Parameter | Type | | Description |
|---|---|---|---|
| `type` | id or name | required | Type id or name. |

### `elements_list`

*query* — Elements, optionally of one category, level and/or type, or only those shown in a view (all the instances of a type visible in it, for example), with a short summary each.

| Parameter | Type | | Description |
|---|---|---|---|
| `category` | string | optional | Category name (`wall`, `door`, `component`…); see `categories_list`. |
| `level` | id or name | optional | Level id or name. |
| `type` | id or name | optional | Type id or name: only its instances. |
| `view` | id or name | optional | View id or name: only the elements the view shows. |

### `element_get`

*query* — An element with all its parameters (`params`, editable with `element_set`) and derived values (length, height, thickness, volume, room area and perimeter; for beams the length after joins and what each end is framed into; for columns and beams the `section`: profile name, area, mass per length with the material's density (null without one), `ix`, `iy`, `wx`, `wy`).

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Element id. |

### `groups_list`

*query* — The project's groups with their members.

No parameters.

### `measure`

*query* — Measure between two points of a view (meters on its plane: plan coordinates in plans): the distance, its horizontal and vertical parts and its angle. When both points lie on lines (`line_a`, `line_b`, each two points) that are parallel, the distance between the lines is given too. Nothing is created.

| Parameter | Type | | Description |
|---|---|---|---|
| `a` | [x, y] | required | First point `[x, y]`. |
| `b` | [x, y] | required | Second point `[x, y]`. |
| `line_a` | [[x, y], [x, y]] | optional | The line the first point was taken on, as two points. |
| `line_b` | [[x, y], [x, y]] | optional | The line the second point was taken on, as two points. |

### `views_list`

*query* — Views (plans, ceiling plans, elevations, sections, 3D, legends, drafting views, schedules, sheets), optionally of one category.

| Parameter | Type | | Description |
|---|---|---|---|
| `category` | string | optional | `floor_plan`, `ceiling_plan`, `three_d`, `elevation`, `section`, `legend`, `drafting`, `schedule`, `diagram` or `sheet`. |

### `view_get`

*query* — A view with all its parameters (`params`, editable with `view_set`), its `scale` denominator and `scale_text`, the scale as the project writes it (`1:50`, or `1/4" = 1'-0"` in imperial projects), and its `sun`: null, or the settings `view_set` takes plus `derived` (azimuth and altitude, `above_horizon`, the current frame's `date` and `time`, `sunrise` and `sunset`, the number of `frames`, `site_known` and whether it is the `conventional` sun), and its `shading` (`ambient` and `shadow`, null for views that take no sun).

| Parameter | Type | | Description |
|---|---|---|---|
| `view` | id or name | required | View id or name. |

### `materials_list`

*query* — Materials with their colors and how many types use each.

No parameters.

### `material_get`

*query* — A material with all its fields (editable with `material_set`).

| Parameter | Type | | Description |
|---|---|---|---|
| `material` | id or name | required | Material name (or index in `materials_list`). |

### `schedule_table`

*query* — The table of a schedule view: headers, one row per element (with its id) and totals, formatted with the project units.

| Parameter | Type | | Description |
|---|---|---|---|
| `view` | id or name | required | Schedule view id or name. |

## Creating elements

### `wall_create`

*edit* — Create a wall from `start` to `end` on a level: straight, or an arc through `mid` (or with a `bulge` or a `radius`). Its top is `height` above its base, or attached to `top_level` (plus `top_offset`).

| Parameter | Type | | Description |
|---|---|---|---|
| `end` | [x, y] | required | End point of the wall line. |
| `start` | [x, y] | required | Start point of the wall line. |
| `base_offset` | number | optional | Base offset from the level, meters. |
| `bulge` | number | optional | Arc walls: tan(sweep / 4), positive when the arc turns counter-clockwise from `start` to `end` (1 is a half circle; at most 10 either way). Ignored when `mid` is given. |
| `height` | number | optional | Unconnected height in meters (default 3; 10'-0" in imperial projects). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `location_line` | string | optional | Which line of the wall `start`-`end` is: `Centerline` (default), `CoreCenter`, `FinishExterior`, `FinishInterior`, `CoreExterior` or `CoreInterior`. |
| `major_arc` | boolean | optional | With `radius`: take the longer of the two arcs (default: the shorter). |
| `mid` | [x, y] | optional | A point on the arc between the ends: makes an arc wall through the three points. |
| `radius` | number | optional | Arc walls: the radius in meters, positive when the arc turns counter-clockwise from `start` to `end`, negative clockwise; at least half the distance between the ends. Ignored when `mid` or `bulge` is given. |
| `structural` | boolean | optional | A structural wall (default: as the wall type says). |
| `top_level` | id or name | optional | Level the top is attached to, instead of a height. |
| `top_offset` | number | optional | Offset of the top from `top_level`, meters. |
| `type` | id or name | optional | Wall type id or name (default: the default wall type). |

### `curtain_type_create`

*edit* — Create a curtain wall type: walls of it are a grid of panels and mullions. `curtain` changes the defaults (vertical and horizontal lines every 1.5 m, glass panels, 5 x 15 cm mullions inside and around; in imperial projects lines every 5'-0" and 2" x 6" mullions) and is shaped like `params.curtain` in `type_get`: `vertical` and `horizontal` {layout: `None`, `FixedDistance`, `FixedNumber` or `MaxSpacing`, spacing, count, justification: `Start`, `Center` or `End`}, `panel` {kind: `Glass` or `Solid`, thickness, material} or {kind: `Empty`} or {kind: `Component`, component: type id}, and `vertical_mullion`, `horizontal_mullion`, `border_mullion` {width, depth, material} or null for none. Place walls of it with `wall_create`.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Type name (unique among wall types). |
| `curtain` | JSON | optional | Changes to the default grid, panel and mullions. |

### `curtain_get`

*query* — Show a curtain wall's grid: its interior lines (`vertical`: distances along the wall from its start; `horizontal`: heights above its base), the panels by column and row (from the start and from the base; merged cells give their lowest) with their extent, kind and whether they are swapped, and its panel count, glazed area and mullion length.

| Parameter | Type | | Description |
|---|---|---|---|
| `element` | integer | required | The curtain wall's id. |

### `curtain_grid_set`

*edit* — Adjust one curtain wall's grid on one axis. Lines are numbered from 0 in the order `curtain_get` lists them (`vertical` or `horizontal`); positions are distances along the wall from its start for vertical lines and heights above the base for horizontal ones. In order: `reset` goes back to the type's lines; `lines` gives all of them; `move` moves lines `[index, to]`; `remove` takes lines out; `add` puts lines in. Any of these gives the wall its own lines (later type changes no longer move them). Segments are a line's pieces between the lines across it (`[line, segment]`, segment numbered from the base or from the start, as the grid was before this command): `remove_segments` merges the cells on both sides into one panel with no mullion between, `restore_segments` puts them back.

| Parameter | Type | | Description |
|---|---|---|---|
| `axis` | JSON | required | Which lines: `Vertical` or `Horizontal`. |
| `element` | integer | required | The curtain wall's id. |
| `add` | [number, …] | optional | Positions of new lines. |
| `lines` | [number, …] | optional | All the interior lines, replacing the current ones. |
| `move` | [[integer or number, …], …] | optional | Lines to move, as `[index, new position]`. |
| `remove` | [id, …] | optional | Indices of lines to remove. |
| `remove_segments` | [[id, …], …] | optional | Segments to remove, as `[line, segment]`. |
| `reset` | boolean | optional | Go back to the type's lines (and drop removed segments of this axis). |
| `restore_segments` | [[id, …], …] | optional | Removed segments to put back, as `[line, segment]`. |

### `curtain_panel_set`

*edit* — Swap panels of one curtain wall: the panels of cells `[column, row]` (numbered from the start and from the base, as `curtain_get` lists them; a cell of merged cells swaps all of them) become `panel`, or the type's again when `panel` is null.

| Parameter | Type | | Description |
|---|---|---|---|
| `cells` | [[id, …], …] | required | The cells, as `[column, row]`. |
| `element` | integer | required | The curtain wall's id. |
| `panel` | object | optional | The new panel (null: the type's). |

### `door_create`

*edit* — Place a door in a wall (centered on the wall unless `offset` or `point` is given).

| Parameter | Type | | Description |
|---|---|---|---|
| `flip_facing` | boolean | optional | Face the other side of the wall. |
| `flip_hand` | boolean | optional | Hinge on the other jamb. |
| `offset` | number | optional | Distance along the wall from its start to the opening's center, meters. |
| `point` | [x, y] | optional | A point near the wall: the opening is centered at its projection, on its side. |
| `sill` | number | optional | Sill height (default: the type's default sill for windows, 0 for doors). |
| `type` | id or name | optional | Door or window type id or name (default: the default type). |
| `wall` | integer | optional | Id of the host wall; omit it to use the wall nearest `point`. |

### `door_type_create`

*edit* — Create a door type. `style` is `swing` (a solid leaf, the default) or `french` (a French door: glazed leaves of stiles and rails around a pane, over a solid bottom rail `bottom_rail` tall, a parameter of the type); `leaves` is 1 (hinged on the left; flip an element's hand for the right) or 2 (a double door, leaves hinged on either jamb meeting in the middle, drawn with two swings in plans and written to IFC as `DOUBLE_DOOR_SINGLE_SWING`). Doors stand on the floor (sill 0). Place them with `door_create`.

| Parameter | Type | | Description |
|---|---|---|---|
| `bottom_rail` | number | optional | French doors: height of the solid bottom rail of each leaf, meters (default 0.4; 1'-4" in imperial projects). |
| `glass` | string | optional | French doors: material of the panes (default: the template's glass, if the project has it). |
| `height` | number | optional | Height of the opening, meters (default 2.1, French doors 2.2; 6'-8" in imperial projects). |
| `leaves` | integer | optional | Number of leaves: 1 (default) or 2. |
| `material` | string | optional | Material of the leaves (default: the template's wood, if the project has it). |
| `name` | string | optional | Type name, unique among door types (default: the component's name and size, in centimeters; in feet and inches in imperial projects). |
| `style` | string | optional | `swing` (default) or `french`. |
| `width` | number | optional | Width of the opening, meters (default 0.8 for one leaf, 1.4 for two; 2'-8" and 5'-0" in imperial projects). |

### `window_create`

*edit* — Place a window in a wall (centered on the wall unless `offset` or `point` is given).

| Parameter | Type | | Description |
|---|---|---|---|
| `flip_facing` | boolean | optional | Face the other side of the wall. |
| `flip_hand` | boolean | optional | Hinge on the other jamb. |
| `offset` | number | optional | Distance along the wall from its start to the opening's center, meters. |
| `point` | [x, y] | optional | A point near the wall: the opening is centered at its projection, on its side. |
| `sill` | number | optional | Sill height (default: the type's default sill for windows, 0 for doors). |
| `type` | id or name | optional | Door or window type id or name (default: the default type). |
| `wall` | integer | optional | Id of the host wall; omit it to use the wall nearest `point`. |

### `niche_type_create`

*edit* — Create a niche type: a recess `width` wide, `height` tall and `depth` deep in one face of a wall, its bottom `sill` above the wall's base by default. In a wall thinner than the depth plus 1 cm the niche stops 1 cm short of the other face (an opening through a wall is a door or a window). Place niches with `niche_create`.

| Parameter | Type | | Description |
|---|---|---|---|
| `depth` | number | optional | How far it goes into the wall from its face, meters (default 0.1; 4" in imperial projects). |
| `height` | number | optional | Height, meters (default 0.6; 2'-0" in imperial projects). |
| `name` | string | optional | Type name, unique among niche types (default: the component's name and its width, height and depth, in centimeters; in feet and inches in imperial projects). |
| `sill` | number | optional | Height of its bottom above the wall's base in new niches, meters (default 0.9; 3'-0" in imperial projects; 0 for a niche from the floor). |
| `width` | number | optional | Width along the wall, meters (default 0.6; 2'-0" in imperial projects). |

### `niche_create`

*edit* — Place a niche in a wall: a recess in one of its faces, centered on the wall unless `offset` or `point` is given. It is in the wall's left face (seen from its start to its end), in the right one with `flip_facing`, and with `point` in the face on the point's side. A project with no niche type gets a default one (see `niche_type_create`) in the same undo step. The wall keeps standing behind the niche: it is not split, and rooms, joins and framing ignore it. A niche that would overlap a door, a window or another niche of its wall cuts nothing; `element_get` then shows no `volume`.

| Parameter | Type | | Description |
|---|---|---|---|
| `flip_facing` | boolean | optional | In the wall's right face instead of its left one. |
| `offset` | number | optional | Distance along the wall from its start to the niche's center, meters. |
| `point` | [x, y] | optional | A point near the wall: the niche is centered at its projection, in the face on its side. |
| `sill` | number | optional | Height of its bottom above the wall's base, meters (default: the type's). |
| `type` | id or name | optional | Niche type id or name (default: the first niche type; one is made when the project has none). |
| `wall` | integer | optional | Id of the host wall; omit it to use the wall nearest `point`. |

### `floor_create`

*edit* — Create a floor slab from an outline (or the loop of walls).

| Parameter | Type | | Description |
|---|---|---|---|
| `boundary` | [[x, y], …] | optional | Closed outline (at least 3 points); omit with `from_walls` or `curves`. |
| `curves` | [JSON, …] | optional | The outline drawn with curves (lines, arcs, circles, ellipses, rectangles, polygons) instead of `boundary`; loops inside the outline are openings (floors and roofs). The element keeps the curves (see `sketch_get`). |
| `from_walls` | boolean | optional | Follow the outer boundary of the walls of the level, leaving interior walls out (exterior faces; interior faces for ceilings). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `offset` | number | optional | Floors: top above the level (default 0). Ceilings: bottom above the level (default 2.6; 8'-0" in imperial projects). Roofs: base above the level (default: top of the tallest wall). |
| `openings` | [[[x, y], …], …] | optional | Openings through the slab (shafts, stairwells): closed outlines inside the boundary, apart from each other. |
| `structural` | boolean | optional | A structural slab, one that takes reinforcement (default false). |
| `type` | id or name | optional | Type id or name (default: the category's default type). |
| `wall_face` | string | optional | With `from_walls`: the face of each wall the outline follows, `FinishExterior`, `FinishInterior`, `CoreExterior`, `CoreInterior`, `CoreCenter` or `Centerline`, where exterior is the side outside the loop of walls and the core is a wall's structure layers. Each side goes to that face of its own wall. Without it the outline is the walls' boundary moved out (in, for ceilings) by half the thickest wall. |

### `floor_opening_add`

*edit* — Add an opening (a shaft, a stairwell) to a floor: a closed outline inside its boundary, apart from its other openings.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Floor id. |
| `outline` | [[x, y], …] | required | Outline of the opening, at least 3 points. |

### `floor_opening_remove`

*edit* — Remove one opening of a floor (by its index in the floor's `openings`).

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Floor id. |
| `index` | integer | required | Opening index, from 0. |

### `ceiling_create`

*edit* — Create a ceiling from an outline (or the inside of the loop of walls).

| Parameter | Type | | Description |
|---|---|---|---|
| `boundary` | [[x, y], …] | optional | Closed outline (at least 3 points); omit with `from_walls` or `curves`. |
| `curves` | [JSON, …] | optional | The outline drawn with curves (lines, arcs, circles, ellipses, rectangles, polygons) instead of `boundary`; loops inside the outline are openings (floors and roofs). The element keeps the curves (see `sketch_get`). |
| `from_walls` | boolean | optional | Follow the outer boundary of the walls of the level, leaving interior walls out (exterior faces; interior faces for ceilings). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `offset` | number | optional | Floors: top above the level (default 0). Ceilings: bottom above the level (default 2.6; 8'-0" in imperial projects). Roofs: base above the level (default: top of the tallest wall). |
| `type` | id or name | optional | Type id or name (default: the category's default type). |
| `wall_face` | string | optional | With `from_walls`: the face of each wall the outline follows, `FinishExterior`, `FinishInterior`, `CoreExterior`, `CoreInterior`, `CoreCenter` or `Centerline`, where exterior is the side outside the loop of walls and the core is a wall's structure layers. Each side goes to that face of its own wall. Without it the outline is the walls' boundary moved out (in, for ceilings) by half the thickest wall. |

### `roof_create`

*edit* — Create a roof over an outline (or the loop of walls): flat by default; change its shape (shed, gable, hip) with `element_set`.

| Parameter | Type | | Description |
|---|---|---|---|
| `boundary` | [[x, y], …] | optional | Closed outline (at least 3 points); omit with `from_walls` or `curves`. |
| `curves` | [JSON, …] | optional | The outline drawn with curves (lines, arcs, circles, ellipses, rectangles, polygons) instead of `boundary`; loops inside the outline are openings (floors and roofs). The element keeps the curves (see `sketch_get`). |
| `from_walls` | boolean | optional | Follow the outer boundary of the walls of the level, leaving interior walls out (exterior faces; interior faces for ceilings). |
| `holes` | [[[x, y], …], …] | optional | Openings through the roof (courtyards): closed outlines inside the boundary, apart from each other. Pitched roofs slope down to their eaves too. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `offset` | number | optional | Floors: top above the level (default 0). Ceilings: bottom above the level (default 2.6; 8'-0" in imperial projects). Roofs: base above the level (default: top of the tallest wall). |
| `overhang` | number | optional | Overhang beyond the outline, meters (default 0.5). |
| `type` | id or name | optional | Type id or name (default: the category's default type). |
| `wall_face` | string | optional | With `from_walls`: the face of each wall the outline follows, `FinishExterior`, `FinishInterior`, `CoreExterior`, `CoreInterior`, `CoreCenter` or `Centerline`, where exterior is the side outside the loop of walls and the core is a wall's structure layers. Each side goes to that face of its own wall. Without it the outline is the walls' boundary moved out (in, for ceilings) by half the thickest wall. |

### `roof_edge_set`

*edit* — Set the slope of one edge of a gable or hip roof's outline (edge `i` runs from outline point `i` to the next): `0` makes it a gable end, a positive slope an eave rising at that slope, `null` returns it to the shape's own.

| Parameter | Type | | Description |
|---|---|---|---|
| `edge` | integer | required | Edge index, from 0. |
| `id` | integer | required | Roof id. |
| `slope` | number | optional | Slope as rise over run (0.5 = 50 %), `0` for a gable end, or `null` for the roof's slope. |

### `roof_opening_remove`

*edit* — Remove one opening of a roof (by its index in the roof's `holes`).

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Roof id. |
| `index` | integer | required | Opening index, from 0. |

### `room_create`

*edit* — Create a room at a point enclosed by room-bounding walls (its boundary follows the walls).

| Parameter | Type | | Description |
|---|---|---|---|
| `point` | [x, y] | required | A point inside the room. |
| `height` | number | optional | Upper limit above the level (default: the level's height). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `name` | string | optional | Default: the room default name. |
| `number` | string | optional | Default: the next free number. |

### `grid_create`

*edit* — Create a grid line. Without `name` it takes the next name of its series: the one after the most recently placed grid parallel to it (`1` → `2`, `A` → `B`, `Z` → `AA`, `A1` → `A2`, `1.1` → `1.2`); a grid across the existing ones starts a new series (letters across numbered grids, numbers otherwise). Names given this way are unique among the grids.

| Parameter | Type | | Description |
|---|---|---|---|
| `end` | [x, y] | required | End point. |
| `start` | [x, y] | required | Start point (where the bubble is). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `name` | string | optional | Grid name (default: the next name of its series). |

### `column_create`

*edit* — Place a column; its top goes to the level above (or `height` when there is none).

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Center of the column in plan. |
| `height` | number | optional | Unconnected height (default: the level's height). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `rotation` | number | optional | Rotation in plan, degrees. |
| `structural` | boolean | optional | Load-bearing (default true); an architectural column is cut by the walls it is joined with. |
| `top_level` | id or name | optional | Level the top is attached to (default: the level above). |
| `top_offset` | number | optional | Offset of the top from `top_level`, meters. |
| `type` | id or name | optional | Type id or name (default: the category's default type). |

### `beam_create`

*edit* — Create a beam from `start` to `end`, straight or, for an arc beam, through `mid` (or with a `bulge` or a `radius`); its location line is `top_offset` above the level (default: the level's height), at the top of the section unless `vertical` says otherwise. An arc beam is level and joins columns and other beams at its ends like a straight one.

| Parameter | Type | | Description |
|---|---|---|---|
| `end` | [x, y] | required | End of the beam axis. |
| `start` | [x, y] | required | Start of the beam axis. |
| `bulge` | number | optional | Arc beams: `tan(sweep / 4)`, positive when the arc turns counter-clockwise from `start` to `end` (1 is a half circle; at most 10 either way). Ignored when `mid` is given. |
| `end_offset` | number | optional | Elevation of the location line at the end, when it differs from the start (a sloped beam). |
| `lateral` | string | optional | Line across the section the axis follows: `center` (default), `left` or `right` face. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `major_arc` | boolean | optional | With `radius`: take the longer of the two arcs (default: the shorter). |
| `mid` | [x, y] | optional | Arc beams: a point the location line passes through between `start` and `end`. |
| `radius` | number | optional | Arc beams: the radius in meters, positive when the arc turns counter-clockwise from `start` to `end`, negative when clockwise. Ignored when `mid` or `bulge` is given. |
| `top_offset` | number | optional | Elevation of the location line above the level, meters (default: the level height). |
| `type` | id or name | optional | Type id or name (default: the category's default type). |
| `vertical` | string | optional | Height of the section at the location elevation: `top` (default), `center` or `bottom`. |

### `columns_at_grids`

*edit* — Place a column at every intersection of grid lines on a level, skipping places that already have a column. Returns the new ids (`created`) and how many intersections were skipped.

| Parameter | Type | | Description |
|---|---|---|---|
| `grids` | [id, …] | optional | Grid ids (default: every grid). |
| `height` | number | optional | Unconnected height (default: the level's height). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `rotation` | number | optional | Rotation in plan, degrees. |
| `structural` | boolean | optional | Load-bearing (default true). |
| `top_level` | id or name | optional | Level the tops are attached to (default: the level above). |
| `top_offset` | number | optional | Offset of the tops from `top_level`, meters. |
| `type` | id or name | optional | Column type id or name (default: the category's default type). |

### `beams_on_grids`

*edit* — Place beams along grid lines: on each grid, a beam between each pair of neighboring columns of the level whose centers lie on it. Spans that already have a beam are skipped.

| Parameter | Type | | Description |
|---|---|---|---|
| `grids` | [id, …] | optional | Grid ids (default: every grid). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `top_offset` | number | optional | Top of the beams above the level, meters (default: the level height). |
| `type` | id or name | optional | Beam type id or name (default: the category's default type). |

### `beams_between_columns`

*edit* — Place beams from column to column in the order given (a chain), on the level of the first column. Spans that already have a beam are skipped.

| Parameter | Type | | Description |
|---|---|---|---|
| `columns` | [id, …] | required | Column ids, at least two, in the order to connect. |
| `top_offset` | number | optional | Top of the beams above the level, meters (default: the level height). |
| `type` | id or name | optional | Beam type id or name (default: the category's default type). |

### `grid_frame_create`

*edit* — Frame a range of levels on the grids: on every storey (each pair of consecutive levels from `base_level` to `top_level`), a column at each grid intersection of the lower level with its top attached to the upper one, and beams on the upper level along each grid between neighboring columns. Columns and beams already modeled are skipped. With `secondary`, each bay between two grids parallel to `parallel_to` and two crossing grids gets beams parallel to it, at most `spacing` apart. One undo step; returns the new `columns`, `beams` and `secondary` beams and how many were `skipped` (and bays `skipped_bays`).

| Parameter | Type | | Description |
|---|---|---|---|
| `base_level` | id or name | optional | The lowest level (default: the lowest level of the project). |
| `beam_type` | id or name | optional | Beam type id or name (default: the first of `material`, made when missing). |
| `column_type` | id or name | optional | Column type id or name (default: the first of `material`, made when missing). |
| `grids` | [id, …] | optional | Grid ids (default: every grid). |
| `material` | string | optional | `concrete`, `steel` (default) or `timber`: the default types' material. |
| `secondary` | object | optional | Secondary beams in each bay. |
| `top_level` | id or name | optional | The highest level (default: the level above the base). |

### `slab_join_supports`

*edit* — Join each slab with the structure that carries it: beams, structural columns and structural walls whose bodies overlap it (default: every floor, or every floor of `level`). Beams and columns cut the slab, the slab cuts walls, and the shared volume counts once. Pairs already joined are left as they are.

| Parameter | Type | | Description |
|---|---|---|---|
| `floors` | [id, …] | optional | Floor ids (default: every floor, or those of `level`). |
| `level` | id or name | optional | Level id or name, when no floors are given. |

### `structure_number`

*edit* — Number the marks of beams, columns and pad footings (default: all three) in reading order: horizontal beams top to bottom then left to right, the other beams left to right then top to bottom, columns and footings top to bottom then left to right. Marks are a prefix per category (`V`, `P`, `S` in Portuguese; `B`, `C`, `F` in English) and a number.

| Parameter | Type | | Description |
|---|---|---|---|
| `categories` | [string, …] | optional | Categories to number: `beam`, `column` and/or `foundation` (default: all three). |
| `level` | id or name | optional | Level id or name (default: every level, numbered as one sequence per category). |

### `brace_create`

*edit* — Create a brace: a member between two points in space (`[x, y, z]`, `z` above the level), its axis through both. Its ends must not be one above the other.

| Parameter | Type | | Description |
|---|---|---|---|
| `end` | [number, …] | required | End point `[x, y, z]`. |
| `start` | [number, …] | required | Start point `[x, y, z]`. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `type` | id or name | optional | Beam type id or name (default: the category's default type). |

### `braces_between_columns`

*edit* — Brace the bay between two columns: `x` (default) crosses two members from the base of each column to the top of the other, `diagonal` places one from the base of the first to the top of the second.

| Parameter | Type | | Description |
|---|---|---|---|
| `columns` | [id, …] | required | The two column ids. |
| `pattern` | string | optional | `x` or `diagonal`. |
| `type` | id or name | optional | Beam type id or name (default: the category's default type). |

### `truss_create`

*edit* — Lay out a parametric plane truss from `start` to `end`: a `flat` (default), `mono` or `gable` outline `height` deep (the rise at the apex or high end for mono and gable), `panels` bottom-chord panels and the web of `pattern` (`pratt`, the default, `howe`, `warren`, or for gables `kingpost`, `queenpost` and `fink`; a gable-only pattern on another shape is laid out as Pratt). Chords and diagonals are beams, verticals are columns, all ordinary elements the truss keeps in step with its parameters (`element_set` regenerates them, keeping their ids); a member edited or deleted by hand is detached and left alone. Each chord is one continuous beam per straight run (two top chords on a gable). Verticals stand between the chords: on the faces of solid chords, or inside a channel that opens toward the web, and they are turned so the side nearest the chords' width lies across the truss. Where the truss ends on a vertical (both ends of a flat truss, the high end of a mono one) the chords run on over it and the vertical at the far end is turned around, so the ends are closed. `copies` trusses are made `spacing` apart to the left of start → end. Default types come from the profile catalog for the `material` (steel: `Ue 150x60x20x2,00` chords laid on their sides, the bottom one open upward and the top one downward, `Ue 100x50x17x2,00` diagonals laid the same way and posts inside the chords; timber: `Peça 6x16` chords, `Peça 6x12` web and posts), made when missing; concrete is refused. Out-of-range values are refused. One undo step; returns the truss's `id` and its `members`.

| Parameter | Type | | Description |
|---|---|---|---|
| `end` | [x, y] | required | End of the span line in plan (the other support), 0.5 to 200 m from the start. |
| `height` | number | required | Chord axis to chord axis (flat), or the rise at the apex or high end (mono, gable), meters, 0.1 to 20. |
| `start` | [x, y] | required | Start of the span line in plan (a support). |
| `bottom_chord_type` | id or name | optional | Beam type of the bottom chord (default: `chord_type`, else the catalog chord). |
| `bottom_offset` | number | optional | Elevation of the bottom chord's axis above the level (default: the level height minus `height`). |
| `chord_type` | id or name | optional | Beam type of both chords (default: the material's catalog chord). |
| `copies` | integer | optional | How many trusses, 1 to 200 (default 1). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `material` | string | optional | `steel` (default) or `timber`: the catalog rows of the types not given (a given `chord_type` does not change it, so give `material` with a timber chord). |
| `panels` | integer | optional | Bottom-chord panels, 2 to 64, even on a gable (default: about one per 1.5 m). |
| `pattern` | string | optional | `pratt` (default), `howe`, `warren`, `kingpost`, `queenpost` or `fink`. |
| `post_type` | id or name | optional | Column type of the verticals (default: a column type of the material's catalog web row). |
| `shape` | string | optional | `flat` (default), `mono` (rising toward `end`) or `gable` (rising toward mid-span). |
| `spacing` | number | optional | Distance between copies, to the left of start → end, meters, 0.3 to 20 (default 1.5). |
| `top_chord_type` | id or name | optional | Beam type of the top chord (default: `chord_type`, else the catalog chord). |
| `web_type` | id or name | optional | Beam type of the diagonals (default: the material's catalog web row). |

### `bracing_create`

*edit* — Brace one bay of a frame, storey by storey: the bay runs from `start` to `end` in plan (the axes of its two columns), or between two `columns`, whose level, base and height it then takes. `pattern` is `x` (default: both diagonals), `diagonal` (one, rising toward `end`; `flip` turns it, `alternate` turns every other storey), `v` (from the top corners down to the middle of the bay), `chevron` (from the bottom corners up to the middle, an inverted V) or `k` (from the mid-height of each column to the middle of the bay at the top and at the bottom of the storey). `stories` storeys of `height` are braced one above the other from `base_offset`. The braces are ordinary beams the bracing keeps in step with its parameters (`element_set` regenerates them, keeping their ids); a brace edited or deleted by hand is detached and left alone. The default brace type comes from the profile catalog for the `material` (steel: `SHS 100x100x4`; timber: `Peça 6x12`), made when missing; concrete needs a `type`. Out-of-range values are refused. One undo step; returns the bracing's `id` and its `members`.

| Parameter | Type | | Description |
|---|---|---|---|
| `alternate` | boolean | optional | `diagonal` braces change direction storey by storey. |
| `base_offset` | number | optional | The foot of the lowest storey above the level, meters (default: the columns' base, else 0). |
| `columns` | [id, …] | optional | The two column ids of the bay, instead of `start` and `end`. |
| `end` | [x, y] | optional | The other column axis, 0.5 to 200 m from the start (not with `columns`). |
| `flip` | boolean | optional | A `diagonal` rises toward the start instead of the end. |
| `height` | number | optional | One storey, meters, 0.5 to 50 (default: the columns' common height over `stories`, else the level's height). |
| `level` | id or name | optional | Level id or name (default: the first column's level, else the lowest level). |
| `material` | string | optional | `steel` (default) or `timber`: the catalog row of the brace type when `type` is not given. |
| `pattern` | string | optional | `x` (default), `diagonal`, `v`, `chevron` or `k`. |
| `start` | [x, y] | optional | One column axis of the bay in plan (not with `columns`). |
| `stories` | integer | optional | Storeys braced one above the other, 1 to 100 (default 1). |
| `type` | id or name | optional | Beam type of the braces, id or name (default: the material's catalog row). |

### `rebar_create`

*edit* — Add a set of bars to a beam or column: `layout` is `top` or `bottom` (beams), `perimeter` (columns) with `count` bars, `sides` (beams: `count` side bars on each side face, or as many as keep them `spacing` apart over the height), or `stirrups` every `spacing`. The bars are `cover` inside the faces (default 2.5 cm) and follow the host: along an arc beam they follow the arc, each at its own radius, and the stirrups are radial. Top and bottom bars that do not fit side by side (a clear distance of the bar diameter, at least 2 cm) go in a second layer. Bars along a beam longer than `max_length` are made of pieces overlapping by `lap`, the splices of neighbouring bars staggered. A column's `perimeter` bars are `count` spread around its section, or given face by face with `faces` (a bar count for each face of the section, its two corner bars included, the corner bars shared between faces; the column's `derived.faces` lists the faces in order) or `face_spacing`; on a curved face they stand at even angles. Its `stirrups` follow the section's outline, and `ties` adds cross-ties every `spacing`: a hooked bar from each bar between the corners to the bar opposite. With `max_length` a column's vertical bars are cut in lifts that lap by `lap`. `same_type` gives the set to every column (or beam) of the host's type.

| Parameter | Type | | Description |
|---|---|---|---|
| `host` | integer | required | Beam or column id. |
| `layout` | string | required | `top`, `bottom`, `perimeter`, `sides`, `stirrups` or `ties`. |
| `count` | integer | optional | Number of longitudinal bars (default: 2 in beams, 4 in columns), or of side bars on each side face (default 1, unless `spacing` is given). |
| `cover` | number | optional | Concrete cover, meters (default 0.025; 1 1/2" in imperial projects). |
| `face_spacing` | number | optional | Columns, `perimeter`: instead of `faces`, the largest distance between bars on every face, meters. |
| `faces` | [id, …] | optional | Columns, `perimeter`: the number of bars on each face of the section, in the order of the column's `derived.faces`, the two corner bars of a face included (at least 2). |
| `lap` | number | optional | Lap length of the splices, meters (default: 40 bar diameters). |
| `lap_at_joins` | boolean | optional | Beams, bars along the beam: run them half a lap past each end joined to another beam that carries on in the same line (the beams of a ring), so the bars of the two lap there (default false: they stop the cover short of the end). The lap is `lap`, or 40 bar diameters where bars are kept whole (`max_length` 0). |
| `lifts` | [number, …] | optional | Columns, `perimeter`: heights above the column's base where its vertical bars are cut, rising (the construction joints): the bars of each lift run `lap` past the next height. Instead of the equal lifts of `max_length`. |
| `max_length` | number | optional | Longest bar along a beam, meters (default: the stock length, 12 m, or 40' in imperial projects): longer bars are lap spliced. 0 keeps whole bars. |
| `overlapping` | boolean | optional | Columns, `stirrups`: several closed stirrups that overlap, one around each of the rectangles the section is made of (two in an L or a T, three in a U), instead of one following the whole outline. The section must have only square corners. |
| `partition` | string | optional | The part of the structure the sets belong to (e.g. "FOUNDATION", "WALL"), which schedules filter and group by (default: none). |
| `same_type` | boolean | optional | Also give the set to every other column (or beam) of the host's type (default false). |
| `spacing` | number | optional | Stirrup spacing, meters (default 0.15; 6" in imperial projects), or the largest distance between side bars over the height when no `count` is given. |
| `type` | id or name | optional | Bar type id or name (default: the type nearest 10 mm, or 5 mm for stirrups; 1/2" and 3/8" in imperial projects). |

### `rebar_typical`

*edit* — Give beams and columns without reinforcement a typical one: beams two (three when 20 cm wide or more) 10 mm bars at the bottom, two 8 mm at the top (both lap spliced past the stock length) and 5 mm stirrups every 15 cm; columns four 10 mm bars (six in round ones) and 5 mm ties every 15 cm. Default: every beam and column of `level` (or of the project).

| Parameter | Type | | Description |
|---|---|---|---|
| `elements` | [id, …] | optional | Beam and column ids (default: all of `level`, or of the project). |
| `level` | id or name | optional | Level id or name, when no elements are given. |

### `rebar_copy`

*edit* — Copy the reinforcement sets of one host (all, or those of `sets`) to other hosts of the same kind and section in one step: columns and beams of the same type or of types with the same section, walls and strip footings of the same type. Each copy keeps its bar type, layout, cover, splices, partition and hand-given mark. Floors and slab foundations, whose sets keep plan points, are not copied. With `replace`, each target's own sets go first. Nothing is changed when a target does not match. Returns the created set ids and those replaced.

| Parameter | Type | | Description |
|---|---|---|---|
| `from` | integer | required | The host whose sets are copied: a column, beam, wall or strip footing. |
| `to` | [id, …] | required | The hosts that receive them. |
| `replace` | boolean | optional | Delete the targets' own sets first (default false: add beside them). |
| `sets` | [id, …] | optional | The sets of `from` to copy (default all). |

### `wall_rebar_create`

*edit* — Reinforce walls, straight or arc: `horizontal` bars up the wall (ring bars in an arc wall, each as long as the arc at its own radius) and `vertical` bars along it near each face, and `ties` between the faces. With none of the three given, horizontal and vertical bars every 15 cm on both faces. Bars lie `cover` inside the faces of the wall's structure (its core layers), stop at its ends and around its doors and windows, and those longer than `max_length` are lap spliced, neighbouring splices staggered. Horizontal bars may close up towards the base (`zones`); vertical bars may cover part of the height, start between those of another set (`offset`) and have bent ends. Each call adds sets, so a second call makes the additional bars of a zone. One set per family and face, in one undo step; the bars follow the wall.

| Parameter | Type | | Description |
|---|---|---|---|
| `cover` | number | optional | Concrete cover of both faces, meters (default 0.025; 1 1/2" in imperial projects). |
| `cover_exterior` | number | optional | Cover of the exterior face, when it differs. |
| `cover_interior` | number | optional | Cover of the interior face, when it differs. |
| `faces` | string | optional | The faces reinforced: `both` (default), `exterior` or `interior`. |
| `horizontal` | object | optional | The horizontal bars. |
| `lap` | number | optional | Lap length of the splices, meters (default: 40 bar diameters). |
| `lap_at_joins` | boolean | optional | Run the horizontal bars half a lap past each end joined to another wall that carries on in the same line (the walls of a cylinder), so the bars of the two lap there (default false: they meet end to end at the joint). The lap is `lap`, or 40 bar diameters where bars are kept whole (`max_length` 0). |
| `level` | id or name | optional | Level id or name, when no walls are given. |
| `max_length` | number | optional | Longest bar, meters (default: the stock length, 12 m, or 40' in imperial projects): longer bars are lap spliced. 0 keeps whole bars. |
| `outer` | string | optional | The bars nearest the faces: `horizontal` (default) or `vertical`; the other family lies inside them. |
| `partition` | string | optional | The part of the structure the sets belong to (e.g. "FOUNDATION", "WALL"), which schedules filter and group by (default: none). |
| `ties` | object | optional | The ties between the faces, at the horizontal and vertical bars of the exterior face. |
| `vertical` | object | optional | The vertical bars. |
| `walls` | [id, …] | optional | Wall ids (default: the structural walls of `level`, or of the project). |

### `slab_rebar_create`

*edit* — Reinforce slabs: floors and foundation slabs of any boundary. `layout` is `mesh` (default): straight bars in two directions, `angle` and square to it, `spacing` (and `cross_spacing`) apart; or `radial`: ring bars `spacing` apart around `center` and `radial_count` radial bars, with shorter bars from `intermediate` between them. The bars go in the `top` and `bottom` layers (`layers`), `cover` inside the faces of the slab's structure, its edge and its openings: each bar is clipped to the true boundary (arcs and circles as drawn), so every bar has its own length (`element_get` lists a set's `derived.bars` by length). `hook` bends the ends of mesh bars at the slab's edge. Bars longer than `max_length` are lap spliced, neighbouring splices staggered. A slab with openings gets trimming bars around them (`trim`; `false` for none). One set per layer and direction, in one undo step; the bars follow the slab. Without `slabs`, the structural floors (`floor_create` `structural`) and the foundation slabs are reinforced.

| Parameter | Type | | Description |
|---|---|---|---|
| `angle` | number | optional | Direction of the first bars of a mesh, degrees from the plan's X axis (default 0); the second direction is square to it. The first direction is the outer layer. |
| `center` | [x, y] | optional | `radial`: the center of the rings and radial bars (default: the middle of the slab). |
| `cover` | number | optional | Concrete cover, meters (default 0.025; 1 1/2" in imperial projects). |
| `cross_spacing` | number | optional | Distance between the bars of the second direction of a mesh (default: `spacing`). |
| `cross_type` | id or name | optional | Bar type of the second direction of a mesh, or of the radial bars (default: `type`). |
| `hook` | number | optional | Bend at the ends of mesh bars on the slab's edge, meters: positive towards the middle of the slab (down for top bars, up for bottom ones), negative out of it (into the wall above or below). Default 0: straight ends. |
| `intermediate` | number | optional | `radial`: the radius from which a shorter bar runs between each two radial bars (default 0: none). |
| `lap` | number | optional | Lap length of the splices, meters (default: 40 bar diameters). |
| `layers` | string | optional | The layers reinforced: `both` (default), `top` or `bottom`. |
| `layout` | string | optional | `mesh` (default) or `radial`. |
| `level` | id or name | optional | Level id or name, when no slabs are given. |
| `max_length` | number | optional | Longest bar, meters (default: the stock length, 12 m, or 40' in imperial projects): longer bars are lap spliced. 0 keeps whole bars. |
| `partition` | string | optional | The part of the structure the sets belong to (e.g. "FOUNDATION", "BOTTOM SLAB"), which schedules filter and group by (default: none). |
| `radial_count` | integer | optional | `radial`: the number of radial bars (default: as many as keep them `spacing` apart at the edge). |
| `slabs` | [id, …] | optional | Floor and foundation slab ids (default: the structural floors and the foundation slabs of `level`, or of the project). |
| `spacing` | number | optional | Distance between the bars of a mesh, or between ring bars, meters (default 0.15; 6" in imperial projects). |
| `start` | number | optional | `radial`: the radius the radial bars start at, meters (default 0: the center). |
| `trim` | JSON | optional | Trimming bars around openings: their values, or `false` for none (default: two bars along each side reaching 40 diameters past the opening, and a diagonal across each corner). |
| `type` | id or name | optional | Bar type id or name (default: the type nearest 10 mm; 1/2" in imperial projects). |

### `strip_rebar_create`

*edit* — Reinforce strip footings, straight or arc: bars across the footing (`transverse_spacing` apart along it; radial in an arc strip) and bars along it (`longitudinal_spacing` apart across it; ring bars in an arc strip, each as long as the arc at its own radius), in the bottom and top `layers`. A top bar across the footing follows its top, up the slopes of a haunched section and flat between the faces of its beam, and its ends turn down to the bottom layer, closing the edges. The bars along a haunched footing follow the slopes, and leave the beam to its own: `beam` gives it stirrups over the full height of the section, top, bottom and side bars (a haunched strip gets stirrups and two top bars by default; `false` for none). Bars along the strip longer than `max_length` are lap spliced, neighbouring splices staggered. One set per layer and direction, in one undo step; the bars follow the strip.

| Parameter | Type | | Description |
|---|---|---|---|
| `beam` | JSON | optional | The bars of the beam part: their values, or `false` for none. |
| `cover` | number | optional | Concrete cover, meters (default 0.025; 1 1/2" in imperial projects). |
| `lap` | number | optional | Lap length of the splices, meters (default: 40 bar diameters). |
| `lap_at_joins` | boolean | optional | Run the bars along the strip half a lap past each end joined to another strip that carries on in the same line (the strips of a ring footing), so the bars of the two lap there (default false: they stop the cover short of the end). The lap is `lap`, or 40 bar diameters where bars are kept whole (`max_length` 0). |
| `layers` | string | optional | The layers of the footing reinforced: `both` (default), `top` or `bottom`. |
| `level` | id or name | optional | Level id or name, when no strips are given. |
| `longitudinal_spacing` | number | optional | Distance between the bars along the footing, across it, meters (default 0.15; 0 for none). |
| `longitudinal_type` | id or name | optional | Bar type of the bars along the footing (default: `type`). |
| `max_length` | number | optional | Longest bar along the strip, meters (default: the stock length, 12 m, or 40' in imperial projects): longer bars are lap spliced. 0 keeps whole bars. |
| `partition` | string | optional | The part of the structure the sets belong to (e.g. "FOUNDATION", "WALL"), which schedules filter and group by (default: none). |
| `strips` | [id, …] | optional | Strip footing ids (default: those of `level`, or of the project). |
| `transverse_spacing` | number | optional | Distance between the bars across the footing, along it, meters (default 0.15; 6" in imperial projects; 0 for none). |
| `type` | id or name | optional | Bar type id or name of the bars across the footing (default: the type nearest 10 mm; 1/2" in imperial projects). |

### `bar_marks_list`

*query* — Every bar mark of the model, sorted naturally (N2 before N10): its diameter and grade, its shape (`code`, e.g. `L 30-470`, and the legs), the length of one bar (`unit_length`, the mean of its bars), how many bars have it, their total length and weight, the longest bar, the sets using it, and where it comes from (`source`: `frozen`, `override` or `numbered`). Frozen marks no bar uses are listed with a count of 0. Also the marking settings. A mark names one shape: the same diameter, grade, legs and bends within the length tolerance (default 5 mm) and 1 degree, read from either end or mirrored. Lengths in meters, angles in degrees, weights in kilograms.

No parameters.

### `bar_marks_update`

*edit* — Freeze the bar marks: the marks numbered for shapes the frozen marks table does not hold yet are added to it, so they stay when the model changes (entries already there are kept, overrides stay on their sets). With `drop_unused`, entries no bar uses any more are dropped. Changes nothing (and makes no undo step) when there is nothing to freeze or drop. `bar_marks_list` shows the result.

| Parameter | Type | | Description |
|---|---|---|---|
| `drop_unused` | boolean | optional | Drop frozen marks that no bar uses (default false). |

### `bar_marks_renumber`

*edit* — Number the bar marks afresh: the frozen marks table is cleared, and so are the sets' `bar_mark` overrides unless `keep_overrides`; then every shape is numbered from `start` (default: the current one) with `prefix` (default: the current one) in the stable order (partition, diameter descending, length descending, shape) and the numbering is frozen. One undo step; returns the new marks.

| Parameter | Type | | Description |
|---|---|---|---|
| `keep_overrides` | boolean | optional | Keep the marks the sets set themselves (default false: they are cleared). |
| `prefix` | string | optional | Text before the numbers (e.g. "N"; at most 40 characters). |
| `start` | integer | optional | First number (0 to 1000000). |

### `bar_marking_set`

*edit* — Change how bars are marked: the `prefix` and `start` of numbered marks (used by marks numbered from now on; `bar_marks_renumber` renumbers the frozen ones), the length tolerance within which bars are one shape, and the stock length bars are bought in (`bar_length_check` and bending schedules flag longer bars). One undo step.

| Parameter | Type | | Description |
|---|---|---|---|
| `length_tolerance` | number | optional | Legs of bars of one mark differ by at most this, meters (0.0001 to 0.05; default 0.005). |
| `prefix` | string | optional | Text before the numbers (at most 40 characters). |
| `start` | integer | optional | First number (0 to 1000000). |
| `stock_length` | number | optional | Length bars are bought in, meters (1 to 100; default 12). |

### `bar_length_check`

*query* — Bars longer than `max_length` (default: the project's stock length, 12 m unless `bar_marking_set` changed it): the bars that need splices, listed by set and mark with how many there are and the longest. A set with a `max_length` of its own cuts its bars into lapped pieces, which then pass.

| Parameter | Type | | Description |
|---|---|---|---|
| `max_length` | number | optional | Longest bar allowed, meters (default: the stock length). |

### `bending_schedule_create`

*edit* — Create a bending schedule: a reinforcement schedule view with one row per bar mark (mark, diameter, grade, a sketch of the shape with its dimensions, the length of one bar in centimeters, the count, the total length and the weight, from the type's mass per meter or the diameter; rows of bars longer than the stock length noted) and, under it, a summary per diameter and grade (total length and weight; with `waste_percent`, the weight with the waste added). Totals add the bars exactly. It is placed on sheets, exported (`view_export`) and written to CSV (`schedule_export_csv`, shapes as codes such as `L 30-470`) like any schedule; `view_set` changes its `def.bending` options later.

| Parameter | Type | | Description |
|---|---|---|---|
| `group_by_partition` | boolean | optional | Rows under a heading per partition, each partition's marks counted on their own (default false). |
| `hide_totals` | boolean | optional | Hide the totals row of the marks (default false). |
| `name` | string | optional | View name (default: the language's). |
| `partitions` | [string, …] | optional | Only the sets of these partitions (default: every set; "" stands for sets without one). |
| `waste_percent` | number | optional | Waste added in the summary, percent (0 to 100; default: no waste columns). |

### `bar_tag_create`

*edit* — Tag a reinforcement set in a plan, an elevation or a section: a text such as `25 N1 Ø12.5 c/12 C=500` (the count, the mark, the diameter in millimeters, the spacing in centimeters for sets with one, the length of one bar in centimeters, `C=var` when they differ; a set of several marks shows its first and `+`), derived from the set each time; `multiplier` writes `2x` first for a set repeated on both faces. The view draws the set's bars as thin lines (dots where seen end-on), so the tag points at them. The tag goes with its view and its set. A project gets a bar tag type (2.5 mm text) with its first tag; `element_get` gives the text (`derived.text`) and `element_set` changes the tag's `params`.

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Where the text starts (its left end, on the baseline), in the view's drawing coordinates. |
| `set` | integer | required | The reinforcement set (or shaped bars) to tag. |
| `view` | id or name | required | View id or name: a floor or ceiling plan, an elevation or a section. |
| `leader` | [x, y] | optional | Where a leader from the text points (default: no leader). |
| `multiplier` | integer | optional | How many times the set is repeated: 2 writes `2x` first (1 to 100; default 1). |
| `parts` | object | optional | The parts of the text to write, `{count, mark, diameter, spacing, length}`, each true or false (default: all). |
| `rotation_deg` | number | optional | Angle of the text, degrees counter-clockwise (default 0). |
| `type` | id or name | optional | Bar tag type id or name (default: the default bar tag type). |

### `connections_create`

*edit* — Place steel or timber connections of one `kind` at every member end that suits it: `base_plate` under columns; `end_plate` and `timber_bolts` at beam ends that stop at a column; `fin_plate` at beam ends at a column or another beam; `hanger` at beam ends at another beam. Ends that already have a connection are skipped. Plates and bolts follow their member. One undo step; returns the new ids (`created`) and how many ends were `skipped`.

| Parameter | Type | | Description |
|---|---|---|---|
| `kind` | string | required | `base_plate`, `end_plate`, `fin_plate`, `timber_bolts` or `hanger`. |
| `level` | id or name | optional | Level id or name when no `members` are given (default: the active level, else every level); ignored with `members`. |
| `members` | [id, …] | optional | Column ids (base plates) or beam ids (the other kinds); default: every one of `level`. |
| `params` | JSON | optional | Sizes to change from the kind's defaults, meters: `thickness`, `width` and `length` (0: from the member's section plus `edge`), `bolt_diameter`, `rows` and `columns` (1 to 10), `spacing`, `edge`, `anchor_length`. Lengths are 0.001 to 2. |

### `analytical_model`

*query* — The analytical model of the structure: structural columns and beams as members along their center lines between shared nodes. Beams framed into columns end on the column axis, beams ending on beams end on their line, and members are split where others connect. Each member carries its `section`: the profile's name, `width` and `height` (m), `area` (m²), `ix`, `iy` and `j` (m⁴).

No parameters.

### `analytical_export`

*file* — Write the analytical model (as `analytical_model` returns it) to a JSON file.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | required | Path of the .json file to write. |

### `footing_create`

*edit* — Place a pad footing centered at `position`; its top is `top_offset` above the level (default 0).

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Center of the pad in plan. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `rotation` | number | optional | Rotation in plan, degrees. |
| `top_offset` | number | optional | Top above the level, meters (default 0). |
| `type` | id or name | optional | Pad footing type id or name (default: the first pad type). |

### `strip_footing_create`

*edit* — Place a strip footing (a ground beam) centered on the line `start`-`end`, or on an arc through `mid` (or with a `bulge` or a `radius`); its top is `top_offset` above the level (default 0). Strips meet at corners and tees like beams, arcs at the true crossings of their faces.

| Parameter | Type | | Description |
|---|---|---|---|
| `end` | [x, y] | required | End of the strip's center line. |
| `start` | [x, y] | required | Start of the strip's center line. |
| `bulge` | number | optional | Arc strips: tan(sweep / 4), positive when the arc turns counter-clockwise from `start` to `end` (1 is a half circle; at most 10 either way). Ignored when `mid` is given. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `major_arc` | boolean | optional | With `radius`: take the longer of the two arcs (default: the shorter). |
| `mid` | [x, y] | optional | A point on the arc between the ends: makes an arc strip through the three points. |
| `radius` | number | optional | Arc strips: the radius in meters, positive when the arc turns counter-clockwise from `start` to `end`, negative clockwise; at least half the distance between the ends. Ignored when `mid` or `bulge` is given. |
| `top_offset` | number | optional | Top above the level, meters (default 0). |
| `type` | id or name | optional | Strip footing type id or name (default: the first strip type). |

### `foundation_slab_create`

*edit* — Create a foundation slab (a raft) from an outline, from `curves` (lines, arcs, circles: a circular raft keeps its true curve, area and volume) or under the loop of walls of the level; `offset` is its top above the level (default 0).

| Parameter | Type | | Description |
|---|---|---|---|
| `boundary` | [[x, y], …] | optional | Closed outline (at least 3 points); omit with `from_walls` or `curves`. |
| `curves` | [JSON, …] | optional | The outline drawn with curves (lines, arcs, circles, ellipses, rectangles, polygons) instead of `boundary`; loops inside the outline are openings (floors and roofs). The element keeps the curves (see `sketch_get`). |
| `from_walls` | boolean | optional | Follow the outer boundary of the walls of the level, leaving interior walls out (exterior faces; interior faces for ceilings). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `offset` | number | optional | Floors: top above the level (default 0). Ceilings: bottom above the level (default 2.6; 8'-0" in imperial projects). Roofs: base above the level (default: top of the tallest wall). |
| `type` | id or name | optional | Type id or name (default: the category's default type). |
| `wall_face` | string | optional | With `from_walls`: the face of each wall the outline follows, `FinishExterior`, `FinishInterior`, `CoreExterior`, `CoreInterior`, `CoreCenter` or `Centerline`, where exterior is the side outside the loop of walls and the core is a wall's structure layers. Each side goes to that face of its own wall. Without it the outline is the walls' boundary moved out (in, for ceilings) by half the thickest wall. |

### `footings_under_columns`

*edit* — Place a pad footing under each column (default: every column of the level), centered and turned with it, its top at the column's base. Columns that already stand on a pad are skipped.

| Parameter | Type | | Description |
|---|---|---|---|
| `columns` | [id, …] | optional | Column ids (default: every column of `level`). |
| `level` | id or name | optional | Level id or name, when no columns are given (default: the lowest level). |
| `type` | id or name | optional | Pad footing type id or name (default: the first pad type). |

### `strip_footings_under_walls`

*edit* — Place a strip footing under each wall (default: every wall of the level), along its body's centerline (an arc under an arc wall), its top at the wall's base. Walls that already have a strip, and arc walls too tight for the strip's width, are skipped; strips meet at corners like beams.

| Parameter | Type | | Description |
|---|---|---|---|
| `level` | id or name | optional | Level id or name, when no walls are given (default: the lowest level). |
| `type` | id or name | optional | Strip footing type id or name (default: the first strip type). |
| `walls` | [id, …] | optional | Wall ids (default: every wall of `level`). |

### `pile_type_create`

*edit* — Create a pile type: round (`diameter`) or square (`side`), made as `kind` says.

| Parameter | Type | | Description |
|---|---|---|---|
| `diameter` | number | optional | Diameter of a round pile, meters (default 0.4). |
| `kind` | string | optional | How the piles are made: `bored` (default), `cfa` (continuous flight auger), `micro` (micropiles and root piles), `precast` (precast concrete, driven), `steel` (driven), `other`. |
| `material` | string | optional | Material name (default: the first foundation type's, the template's concrete). |
| `name` | string | optional | Type name (default: from the size, e.g. `Estaca Ø40`). |
| `side` | number | optional | Side of a square pile, meters (instead of `diameter`). |

### `pile_create`

*edit* — Create a free pile at `position`: its head (`cut_off`) above the level and its toe given by `toe` or by a `length` below the head. A project without a pile type gets one (round, 40 cm, in concrete).

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Axis of the pile in plan. |
| `cut_off` | number | optional | Head of the pile (its cut-off) above the level, meters (default 0). |
| `embedment` | number | optional | How far the head enters a cap or slab above it, meters (default 0). |
| `length` | number | optional | Length of the pile below its head, meters (default 12); or `toe`. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `rotation` | number | optional | Rotation of a square pile in plan, degrees (default 0). |
| `toe` | number | optional | Toe of the pile above the level, meters (negative: below it); or `length`. |
| `type` | id or name | optional | Pile type id or name (default: the first pile type). |

### `pile_cap_type_create`

*edit* — Create a pile cap type: the layout of its piles (`single`, `pair`, `triangle`, `square`, `line` of `count`, `circle` of `count` on a `radius`, a ring cap with `ring_width`), their spacing and edge distance, the cap's thickness, and the type, length and embedment of its piles. The cap's outline follows: a rectangle around one, two, four and a line of piles; for three, a triangle with its corners cut square; on a circle, a round cap or a ring. A project without a pile type gets one.

| Parameter | Type | | Description |
|---|---|---|---|
| `count` | integer | optional | Number of piles of a `line` (2 to 64) or a `circle` (3 to 64). |
| `edge_distance` | number | optional | Distance from a pile's axis to the cap's edge, meters (default: one pile size). |
| `embedment` | number | optional | How far each pile's head enters the cap, meters (default 0.1). |
| `layout` | string | optional | `single`, `pair` (default), `triangle`, `square`, `line` or `circle`. |
| `material` | string | optional | Material name (default: the first foundation type's, the template's concrete). |
| `name` | string | optional | Type name (default: from the number of piles, e.g. `Bloco 3 estacas`). |
| `pile_length` | number | optional | Length of each pile below its head, meters (default 12). |
| `pile_type` | id or name | optional | The piles' type id or name (default: the first pile type). |
| `radius` | number | optional | Radius of a `circle` of piles (their axes), meters. |
| `ring_width` | number | optional | Width of a ring cap centered on the `circle` of piles, meters (default: a round cap). |
| `spacing` | number | optional | Distance between neighboring piles' axes, meters (default: three pile sizes). |
| `thickness` | number | optional | Thickness of the cap, meters (default 0.8). |

### `pile_cap_create`

*edit* — Place a pile cap centered at `position`, turned by `rotation` (its layout's `u` axis along it), its top `top_offset` above the level. Its piles are made with it, in the same undo step, and follow it: they move, turn and are copied and deleted with it, and change with its type. A project without a pile cap type gets one (two piles).

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Center of the cap (the origin of its layout) in plan. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `rotation` | number | optional | Rotation in plan, degrees (default 0). |
| `top_offset` | number | optional | Top of the cap above the level, meters (default 0). |
| `type` | id or name | optional | Pile cap type id or name (default: the first pile cap type). |

### `pile_caps_under_columns`

*edit* — Place a pile cap under each column (default: every column of the level), centered on it, its top at the column's base; its piles are made with it, all in one undo step. `rotation` turns each cap: `column` (default) as its column, an angle in degrees, or `radial`: its layout's `u` axis pointing away from `center`, so caps under columns on a circle point outward (a three-pile cap's first pile outside). Columns that already stand on a pad footing or a pile cap are skipped.

| Parameter | Type | | Description |
|---|---|---|---|
| `center` | [x, y] | optional | The center radial caps point away from (default: the middle of the columns). |
| `columns` | [id, …] | optional | Column ids (default: every column of `level`). |
| `level` | id or name | optional | Level id or name, when no columns are given (default: the lowest level). |
| `rotation` | number or string | optional | `column` (default), `radial`, or an angle in degrees. |
| `type` | id or name | optional | Pile cap type id or name (default: the first pile cap type). |

### `tie_beams_create`

*edit* — Create tie beams between consecutive pile caps of `caps` (and from the last back to the first when `closed`): straight, or arcs around `center` (each between two caps at the same distance from it, the shorter way), or of a signed `radius` (positive: counter-clockwise). Each beam runs between the caps' centers and stops at their outlines where it frames into their depth; its top is flush with the lower of its two caps' tops unless `top_offset` is given. Tie beams are ordinary beams.

| Parameter | Type | | Description |
|---|---|---|---|
| `caps` | [id, …] | required | Pile cap ids, in order (at least two). |
| `center` | [x, y] | optional | Arc beams: the center of the circle the caps stand on. |
| `closed` | boolean | optional | Also join the last cap to the first. |
| `level` | id or name | optional | Level id or name of the beams (default: the first cap's level). |
| `radius` | number | optional | Arc beams: the radius in meters, positive when the arcs turn counter-clockwise from cap to cap, negative clockwise (ignored with `center`). |
| `top_offset` | number | optional | Top of the beams above their level, meters (default: flush with the caps' tops). |
| `type` | id or name | optional | Beam type id or name (default: the first beam type). |

### `stair_create`

*edit* — Create a stair starting at `start` (the middle of its first riser) and rising along `direction` up to the level above (or `height`). Risers and treads follow the type (the fewest risers within its maximum, and a fixed tread or one from Blondel's formula `2 × riser + tread`) unless `risers` or `tread` set them: the riser height is then the height over the risers. `shape` lays it out straight, with a landing, L- or U-shaped, in three flights round two landings, or around a center. The top floor is the last step: a stair ends with a riser, not a tread.

| Parameter | Type | | Description |
|---|---|---|---|
| `start` | [x, y] | required | Start of the run (center of the first step's edge). |
| `direction` | number | optional | Direction of the first flight in plan, degrees counter-clockwise from +X. |
| `gap` | number | optional | Clear gap between the flights of a U-shaped stair, meters (default 0.1; 4" in imperial projects). |
| `height` | number | optional | Height when there is no level above (default 3; 10'-0" in imperial projects). |
| `inner_radius` | number | optional | Radius of the inner edge of a spiral stair, meters (default 0.15; 6" in imperial projects). |
| `landing` | number | optional | Landing depth along the first flight, meters (default: the stair width). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `number_start` | integer | optional | Number of the first tread (default 1), from -9999 to 9999: another value continues the count of a stair below, for instance. |
| `numbers` | boolean | optional | Number the steps in plans: the treads (winders included, landings not) and the floor the stair arrives at (default false). |
| `posts` | [[string, …], …] | optional | Posts under the landings of an assembled stair, down to its base: a list per landing (up the stair) of places round its edge, named walking onto it (front: where the stair arrives; inner: the side it turns to): `front_outer`, `front`, `front_inner`, `inner`, `back_inner`, `back`, `back_outer`, `outer`. Only places its beams reach take a post (see `stair_check`); none by default. |
| `risers` | integer | optional | Riser count (default: the fewest within the type's maximum riser). |
| `shape` | string | optional | Plan shape: `straight` (default), `straight_landing`, `l_shaped`, `u_shaped`, `three_flight` (up, across and back, with a landing at each corner) or `spiral`. |
| `split` | integer | optional | Risers of the first flight (default: half of them; a third on a three-flight stair). |
| `split2` | integer | optional | Risers of the middle flight of a three-flight stair (default: half of those left). |
| `top_level` | id or name | optional | Level the stair rises to (default: the level above). |
| `tread` | number | optional | Tread depth, meters (default: the type's). |
| `turn` | string | optional | Side the stair turns to, walking up: `left` (default) or `right`. |
| `type` | id or name | optional | Type id or name (default: the category's default type). |
| `winders` | integer | optional | Winder treads in the corner of an L-shaped stair instead of a landing (2 to 6). |

### `stair_check`

*query* — Check a stair's steps: riser count and height, tread on the walking line, Blondel's step `2 × riser + tread` and the rules of its type they break (riser out of range, short tread, step out of the Blondel range, short landing). `landings` lists each landing's elevation and the places round its edge where a post can stand under an assembled stair (where its beams reach the edge), with their plan points and whether one stands there.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Stair element id. |

### `stair_railings_add`

*edit* — Add railings along the sides of a stair or a ramp: they follow its flights, landings and turns (at the pitch of the nosings on stairs), and go with it when it changes, moves or is deleted.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Stair or ramp element id. |
| `inset` | number | optional | Distance in from the edge of the stair, meters (default 0.05). |
| `near` | [x, y] | optional | A point `[x, y]` in plan: without `side`, the railing goes on the side nearest it (where a click picked the stair). |
| `side` | string | optional | `left`, `right` or `both` (default), walking up. |
| `type` | id or name | optional | Railing type id or name (default: the category's default type). |

### `railing_host_set`

*edit* — Put a railing on a stair or ramp (`host`), or take it off (`detach`). On a host it follows the host's `side` (default: the side its path runs nearest), `inset` in from the edge, and moves, turns and goes with it; taken off, it keeps the path it had in plan, at its level plus `base_offset`. A hosted railing can also change only its side or inset.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Railing element id. |
| `detach` | boolean | optional | Take the railing off its host. |
| `host` | integer | optional | Stair or ramp element id to follow. |
| `inset` | number | optional | Distance in from the edge of the host, meters (default 0.05, or the railing's own). |
| `side` | string | optional | `left` or `right`, walking up. |

### `ramp_create`

*edit* — Create a ramp starting at `start` (the middle of its foot) and rising along `direction` up to the level above (or `height`). Flights follow the type's rules: at most its steepest slope and its highest rise per flight, with landings between them; `shape` lays them out in line, L- or U-shaped (switchback), or as a helix. A project without a ramp type gets one.

| Parameter | Type | | Description |
|---|---|---|---|
| `start` | [x, y] | required | Foot of the ramp (middle of its lower edge). |
| `direction` | number | optional | Direction of the first flight in plan, degrees counter-clockwise from +X. |
| `gap` | number | optional | Clear gap between the flights of a U-shaped ramp, meters (default 0.1; 4" in imperial projects). |
| `height` | number | optional | Height when there is no level above (default 3; 10'-0" in imperial projects). |
| `inner_radius` | number | optional | Radius of the inner edge of a spiral ramp, meters (default 1.5; 5'-0" in imperial projects). |
| `landing` | number | optional | Landing length, meters (default: the type's shortest landing). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `shape` | string | optional | Plan shape: `straight` (default), `l_shaped`, `u_shaped` or `spiral`. |
| `slope` | number | optional | Slope of the flights, percent (default: the steepest the type allows). |
| `top_level` | id or name | optional | Level the ramp rises to (default: the level above). |
| `turn` | string | optional | Side the ramp turns to, going up: `left` (default) or `right`. |
| `type` | id or name | optional | Type id or name (default: the category's default type). |

### `ramp_check`

*query* — Check a ramp: its slope, flights (count, rise and run of each) and the rules of its type it breaks (too steep, a flight rising too high, a short landing).

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Ramp element id. |

### `roof_check`

*query* — Check a roof against its covering: the least slope its tiles take and the roof's slopes, the area covered, the number of tiles and their weight, and its framing (purlins, rafters, battens and truss members: lengths, the number of trusses and the volume). The covering and framing are set on the roof's type (`type_set` with `covering` and `framing`).

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Roof element id. |

### `mass_create`

*edit* — Create a conceptual mass: `outline` extruded `height` up from its level (plus `base_offset`), or lofted to another outline (`top`, or the base outline set in by `top_inset`). Mass floors are taken at every level the mass spans unless `floors` lists them. A project without a mass type gets one.

| Parameter | Type | | Description |
|---|---|---|---|
| `base_offset` | number | optional | Offset of the base from the level elevation, meters (default 0). |
| `curves` | [JSON, …] | optional | The base outline drawn with curves (lines, arcs, circles, ellipses, rectangles, polygons) in one loop, instead of `outline`; the mass keeps them. |
| `floors` | [id or name, …] | optional | Levels (ids or names) where mass floors are taken (default: every level the mass spans). |
| `height` | number | optional | Height above the base, meters (default: up to the level above, else 3; 10'-0" in imperial projects). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `outline` | [[x, y], …] | optional | Base outline `[[x, y], …]` in plan, closed (at least three points); or `curves`. |
| `top` | [[x, y], …] | optional | Top outline of a loft `[[x, y], …]` (default: the base outline, an extrusion). |
| `top_inset` | number | optional | Loft to the base outline set in by this distance, meters (instead of `top`). |
| `type` | id or name | optional | Type id or name (default: the category's default type). |

### `sketch_get`

*query* — The curves an element's outline is drawn with: its sketch, or lines along its outline and openings when it has none (floors, ceilings, roofs, masses and the other sketched elements, regions among them).

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Element id. |

### `sketch_set`

*edit* — Draw an element's outline again from curves (edit its boundary): floors, roofs and regions take the loops inside the outline as their openings; ceilings and masses take one loop. The element keeps the curves, so `sketch_get` gives them back. A terrain's measured points can change in the same step (`points`).

| Parameter | Type | | Description |
|---|---|---|---|
| `curves` | [JSON, …] | required | The curves, closed into loops. |
| `id` | integer | required | Element id (a floor, ceiling, roof, mass, shaft, terrain, pad, or a filled or masking region). |
| `points` | [[number, …], …] | optional | Terrain only: measured points `[[x, y, z], …]` replacing the current ones (points that repeat another in plan are dropped). |

### `mass_floors_set`

*edit* — Choose the levels where a mass takes mass floors: `levels` (ids or names), or every level it spans when `levels` is left out; an empty list takes none.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Mass element id. |
| `levels` | [id or name, …] | optional | Levels (ids or names); left out: every level the mass spans. |

### `mass_floors`

*query* — The mass floors of a mass: for each, its level, elevation, area and perimeter (and outline), with the gross floor area and the volume.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Mass element id. |

### `mass_type_create`

*edit* — Create a mass type, shown in `material` (default: the project's first material).

| Parameter | Type | | Description |
|---|---|---|---|
| `material` | string | optional | Material name. |
| `name` | string | optional | Type name (default: the template's mass name). |

### `sweep_create`

*edit* — Create a swept profile: the profile of its type carried along `path` in plan, `base_offset` above the level, on the left of the path walking along it (`flip` puts it on the right); `closed` runs the path back to its first point (around a room). A project without a sweep type gets one (a baseboard).

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | [[x, y], …] | required | Points of the path in plan (at least 2; 3 when closed). |
| `base_offset` | number | optional | Height of the path above the level, meters (default 0). |
| `closed` | boolean | optional | Run the path back to its first point. |
| `flip` | boolean | optional | Put the profile on the right of the path. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `type` | id or name | optional | Type id or name (default: the category's default type). |

### `sweep_type_create`

*edit* — Create a sweep type: its profile drawn as points `[x, y]` (x across, to the left of the path; y up from it, meters) or as `curves` (lines, arcs, circles, rectangles… making one closed outline), shown in `material`. Without a profile it is a baseboard 1.5 cm × 10 cm.

| Parameter | Type | | Description |
|---|---|---|---|
| `curves` | [JSON, …] | optional | The profile drawn with curves instead of points. |
| `material` | string | optional | Material name (default: the project's first material). |
| `name` | string | optional | Type name (default: the template's baseboard name). |
| `profile` | [[x, y], …] | optional | The profile's points, in order (a closed outline that does not cross itself). |

### `sweep_profile_set`

*edit* — Redraw the profile of a sweep type, as points `[x, y]` or `curves`; every sweep of the type follows. The type keeps the curves (`type_get` shows them as its `sketch`), so arcs stay arcs when the profile is edited again.

| Parameter | Type | | Description |
|---|---|---|---|
| `type` | id or name | required | Sweep type id or name. |
| `curves` | [JSON, …] | optional | The profile drawn with curves instead of points. |
| `profile` | [[x, y], …] | optional | The profile's points, in order. |

### `shaft_create`

*edit* — Create a shaft: a vertical opening with `outline` in plan (or drawn with `curves`), from its level plus `base_offset` up to `top_level` (plus `top_offset`) or `height`. Every floor, ceiling and roof it passes through is cut where the outline lies inside it, clear of its own openings; the cuts follow the shaft when it moves, changes or is deleted. A project without a shaft type gets one.

| Parameter | Type | | Description |
|---|---|---|---|
| `base_offset` | number | optional | Bottom above the level, meters (default 0). |
| `curves` | [JSON, …] | optional | The outline drawn with curves (lines, arcs, circles, rectangles…). |
| `height` | number | optional | Height when there is no level above, meters (default 3; 10'-0" in imperial projects). |
| `level` | id or name | optional | Level id or name of the bottom (default: the lowest level). |
| `outline` | [[x, y], …] | optional | Outline `[[x, y], …]` in plan, closed (at least three points); or `curves`. |
| `top_level` | id or name | optional | Level the shaft rises to (default: the highest level above its own). |
| `top_offset` | number | optional | Top above `top_level`, meters (default 0). |
| `type` | id or name | optional | Type id or name (default: the category's default type). |

### `shaft_type_create`

*edit* — Create a shaft type (shafts have no properties of their own; types name them).

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | optional | Type name (default: the template's shaft name). |

### `topography_create`

*edit* — Create a terrain surface (topography): measured `points` `[x, y, z]` inside a closed `boundary` (or one drawn with `curves`; with neither, the convex hull of the points). The points are triangulated with the boundary, whose vertices take the elevations the points give them; the earth closes down to a flat base `base_depth` below the lowest point. Points outside the boundary are kept: they give the boundary its elevations but are not vertices of the surface. A project without a terrain type gets one (and an earth material).

| Parameter | Type | | Description |
|---|---|---|---|
| `base_depth` | number | optional | Depth of the flat base below the lowest point of the surface, meters (default 2; 6'-0" in imperial projects). |
| `boundary` | [[x, y], …] | optional | Boundary `[[x, y], …]` in plan, closed (at least three points); or `curves`. |
| `curves` | [JSON, …] | optional | The boundary drawn with curves (lines, arcs, circles, rectangles…). |
| `level` | id or name | optional | Level id or name the elevations are measured from (default: the lowest level). |
| `points` | [[number, …], …] | optional | Measured points `[[x, y, z], …]`: plan position and elevation above the level, meters. |
| `type` | id or name | optional | Type id or name (default: the category's default type). |

### `topography_points_set`

*edit* — Edit the measured points of a terrain surface: replace them all (`points`), or change elevations (`set`), remove points (`remove`, by index) and add new ones (`add`), in that order. Indices refer to the points before `remove` and `add`: the current ones, or `points` when it is given. Points that repeat another in plan are dropped (the first is kept); the answer says how many.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Terrain element id. |
| `add` | [[number, …], …] | optional | Points `[[x, y, z], …]` to add. |
| `points` | [[number, …], …] | optional | All the points `[[x, y, z], …]`, replacing the current ones. |
| `remove` | [id, …] | optional | Indices of points to remove. |
| `set` | [object, …] | optional | New elevations `[{index, z}, …]`. |

### `topography_type_create`

*edit* — Create a terrain type: the earth `material` sections cut it in, and its contour lines every `contour_interval` meters, every `major_every`-th a major (heavier) one.

| Parameter | Type | | Description |
|---|---|---|---|
| `contour_interval` | number | optional | Spacing of the contour lines, meters (default 1; 1'-0" in imperial projects). |
| `major_every` | integer | optional | Every this many contours, a major one (default 5). |
| `material` | string | optional | Material name (default: the project's earth material, added if missing). |
| `name` | string | optional | Type name (default: the template's terrain name). |

### `pad_create`

*edit* — Create a building pad: a flat area at `elevation` (meters above the terrain's level) inside a closed `outline` (or one drawn with `curves`), graded into a terrain (`host`) with cut and fill slopes out to the existing ground. Slopes are the horizontal run per unit of rise (`cut_slope` 1 is 1:1); without them the pad uses its type's. Pads of one terrain apply in creation order. A project without a pad type gets one.

| Parameter | Type | | Description |
|---|---|---|---|
| `elevation` | number | required | Elevation of the pad above the terrain's level, meters. |
| `curves` | [JSON, …] | optional | The outline drawn with curves (lines, arcs, circles, rectangles…). |
| `cut_slope` | number | optional | Run per unit of rise of the cut slope (default: the type's). |
| `fill_slope` | number | optional | Run per unit of rise of the fill slope (default: the type's). |
| `host` | integer | optional | Terrain element id (default: the terrain on `level` whose boundary holds the outline's center). |
| `level` | id or name | optional | Level id or name where the default terrain is looked for (default: any level). |
| `outline` | [[x, y], …] | optional | Outline `[[x, y], …]` in plan, closed (at least three points); or `curves`. |
| `type` | id or name | optional | Type id or name (default: the category's default type). |

### `pad_type_create`

*edit* — Create a building pad type: the run per unit of rise of its cut and fill slopes.

| Parameter | Type | | Description |
|---|---|---|---|
| `cut_slope` | number | optional | Run per unit of rise of the cut slope (default 1). |
| `fill_slope` | number | optional | Run per unit of rise of the fill slope (default 1.5). |
| `name` | string | optional | Type name (default: the template's pad name). |

### `mass_face_move`

*edit* — Push or pull a face of a mass `distance` meters outward (inward when negative): `top` changes its height, `bottom` its base offset and height, and `side` (with `side`, the index of the base outline's edge from that point to the next) moves that side, the sides beside it keeping their directions. A loft's sides are not moved.

| Parameter | Type | | Description |
|---|---|---|---|
| `distance` | number | required | Distance outward, meters (negative: inward). |
| `face` | string | required | `top`, `bottom` or `side`. |
| `id` | integer | required | Mass element id. |
| `side` | integer | optional | The side's edge of the base outline (with `face: "side"`). |

### `mass_walls_create`

*edit* — Walls by face: a wall over each side of a mass extruded straight up (`sides`: the base outline's edges, by default all), its exterior face on the mass's face, from the mass's base up its height. Walls meeting at the mass's corners join. A loft's slanted sides take no walls.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Mass element id. |
| `sides` | [id, …] | optional | Edges of the base outline to put walls on (default: every edge). |
| `type` | id or name | optional | Wall type id or name (default: the category's default type). |

### `mass_slabs_create`

*edit* — Floors from mass floors: a floor on each mass floor of a mass (or those at `levels`), its outline the mass's at that level and its top at the level.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Mass element id. |
| `levels` | [id or name, …] | optional | Levels (ids or names) of the mass floors to build (default: all of them). |
| `type` | id or name | optional | Floor type id or name (default: the category's default type). |

### `mass_roof_create`

*edit* — Roof by face: a flat roof on the top face of a mass, over its top outline, on the mass's level at the height of its top.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Mass element id. |
| `overhang` | number | optional | How far the eaves reach past the top outline, meters (default 0). |
| `type` | id or name | optional | Roof type id or name (default: the category's default type). |

### `ramp_type_create`

*edit* — Create a ramp type: a slab `thickness` thick and `width` wide, within the accessibility rules given (by default 8.33 % slope, 0.80 m rise per flight and 1.20 m landings).

| Parameter | Type | | Description |
|---|---|---|---|
| `max_flight_rise` | number | optional | Highest rise of one flight, meters (default 0.80; 30" in imperial projects). |
| `max_slope` | number | optional | Steepest slope, percent (default 8.33). |
| `min_landing` | number | optional | Shortest landing, meters (default 1.20; 60" in imperial projects). |
| `name` | string | optional | Type name (default: the template's ramp name). |
| `thickness` | number | optional | Slab thickness, meters (default 0.15; 6" in imperial projects). |
| `width` | number | optional | Width, meters (default 1.2; 4'-0" in imperial projects). |

### `railing_create`

*edit* — Create a railing along a path (at least 2 points).

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | [[x, y], …] | required | Points of the railing path (at least 2). |
| `base_offset` | number | optional | Base above the level, meters. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `type` | id or name | optional | Type id or name (default: the category's default type). |

### `component_place`

*edit* — Place a component (furniture, plumbing fixture, electrical device…) of a component type. Electrical devices default to their usual mounting height (outlets 0.30 m, switches 1.10 m, boards 1.30 m, wall lights 2.10 m, ceiling lights at the level height, data outlets 0.30 m, racks 1.40 m, air terminals on the roof under them, earth electrodes 0.50 m below the level; in imperial projects outlets 1'-6", switches 4'-0", boards and racks 4'-6", wall lights 7'-0") and may be put on a circuit or system of their kind; lightning protection devices join the first lightning protection system by default.

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Insertion point in plan. |
| `type` | id or name | required | Component type id or name. |
| `base_offset` | number | optional | Base above the level, meters (default 0; for devices, their mounting height). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `rotation` | number | optional | Rotation in plan, degrees. |
| `system` | id or name | optional | Circuit or system id, name or code to connect it to. |

## Building services

### `pipe_create`

*edit* — Create a pipe from `start` to `end` (or a vertical riser at `start` up or down to `end_offset`), in a plumbing system. Its center line starts `offset` above the level (default: where it starts on a pipe of its system, such as the end of a run or a riser; else 0.60 m for supply and vent pipes, -0.30 m for drains; 2'-0" and -1'-0" in imperial projects). Drains fall along the flow by `slope` (percent; default 2 % up to 75 mm, 1 % above; in imperial projects 1/4" per foot up to 3", 1/8" above); other pipes are level unless `end_offset` or `slope` says otherwise.

| Parameter | Type | | Description |
|---|---|---|---|
| `start` | [x, y] | required | Start of the center line in plan. |
| `end` | [x, y] | optional | End in plan (default: the start, for a riser). |
| `end_offset` | number | optional | Elevation of the end above the level, meters (overrides `slope`). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `offset` | number | optional | Elevation of the start above the level, meters. |
| `slope` | number | optional | Fall from the start to the end, percent of the plan length. |
| `system` | id or name | optional | Plumbing system id, name or code (e.g. "AF"). |
| `type` | id or name | optional | Pipe type id or name (default: the first pipe type). |

### `conduit_create`

*edit* — Create a conduit from `start` to `end` (or a vertical drop at `start` to `end_offset`) carrying `circuits`. It runs `offset` above the level (default: where it starts on another conduit, such as the end of a run or a vertical drop; else in the slab above, at the level height).

| Parameter | Type | | Description |
|---|---|---|---|
| `start` | [x, y] | required | Start of the center line in plan. |
| `circuits` | [id or name, …] | optional | Circuits it carries: ids, names or codes. |
| `end` | [x, y] | optional | End in plan (default: the start, for a vertical drop). |
| `end_offset` | number | optional | Elevation of the end above the level, meters (default: as the start). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `offset` | number | optional | Elevation of the start above the level, meters. |
| `type` | id or name | optional | Conduit type id or name (default: the first conduit type). |

### `cable_tray_create`

*edit* — Create a cable tray from `start` to `end` (or a vertical run at `start` to `end_offset`) carrying `circuits`. Its center line runs `offset` above the level (default: where it starts on another cable tray, such as the end of a run; else 0.30 m under the level above). Trays meet other trays at their ends or along them (elbows, tees and crosses are derived), not conduits or pipes. A project without a cable tray type gets one (200 × 100 mm, 8" × 4" in imperial projects; ladder).

| Parameter | Type | | Description |
|---|---|---|---|
| `start` | [x, y] | required | Start of the center line in plan. |
| `circuits` | [id or name, …] | optional | Circuits it carries: ids, names or codes. |
| `end` | [x, y] | optional | End in plan (default: the start, for a vertical run). |
| `end_offset` | number | optional | Elevation of the end above the level, meters (default: as the start). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `offset` | number | optional | Elevation of the start above the level, meters. |
| `type` | id or name | optional | Cable tray type id or name (default: the first cable tray type). |

### `cable_tray_type_create`

*edit* — Create a cable tray type: an open channel `width` × `height` (meters; default 0.2 × 0.1, 8" × 4" in imperial projects; at most 2 × 1), built as `style` says (`ladder`, the default, `solid_bottom` or `perforated`), in `material` (default: the project's first metal). The name defaults to its section.

| Parameter | Type | | Description |
|---|---|---|---|
| `height` | number | optional | Outside height, meters. |
| `material` | string | optional | Material name. |
| `name` | string | optional | Type name (default: by its section, e.g. "Cable tray 200×100"). |
| `style` | string | optional | `ladder`, `solid_bottom` or `perforated`. |
| `width` | number | optional | Outside width, meters. |

### `valve_place`

*edit* — Place a valve on a pipe, at `point` (the nearest point of the pipe in plan, within 0.5 m) or `distance` meters along the pipe from its start (default: its middle). `type` picks a valve type; else `kind` (`gate`, `ball`, `check` or `globe`) picks the first type of that kind, created when the project has none; else the first valve type (a gate valve type is created when there is none). The valve takes the pipe's diameter and system and splits its network there; it moves, is copied and is deleted with the pipe.

| Parameter | Type | | Description |
|---|---|---|---|
| `pipe` | integer | required | Pipe element id. |
| `distance` | number | optional | Distance from the pipe's start along it, meters. |
| `kind` | string | optional | `gate`, `ball`, `check` or `globe`. |
| `point` | [x, y] | optional | A point in plan on (or near) the pipe. |
| `type` | id or name | optional | Valve type id or name. |

### `valve_type_create`

*edit* — Create valve types, one per kind in `kinds` (`gate`, `ball`, `check`, `globe`; default: a gate valve), named after their kind (or `name`, for one kind), in `material` (default: the project's first metal).

| Parameter | Type | | Description |
|---|---|---|---|
| `kinds` | [string, …] | optional | Kinds of valve to create a type for. |
| `material` | string | optional | Material name. |
| `name` | string | optional | Type name (only with one kind). |

### `device_type_create`

*edit* — Give the project the built-in component type of a kind of device, if it has none: `device` is `outlet`, `dedicated_outlet`, `switch`, `ceiling_light`, `wall_light`, `panel`, `data_outlet` (with `ports` jacks, default 2), `rack`, `air_terminal` or `earth_electrode`. Returns the type (the existing one when there is one).

| Parameter | Type | | Description |
|---|---|---|---|
| `device` | string | required | Kind of device. |
| `ports` | integer | optional | Data outlets: jacks per outlet (1 to 48). |

### `lps_conductor_create`

*edit* — Create a lightning protection conductor from `start` to `end`, or a down conductor straight down from `start` (no `end`). Without offsets, a conductor lies on the roof under its ends (else where it starts on another conductor, else at the level); a down conductor runs from the roof over `start` (else the level above) down to the earthing, 0.5 m below the level. `role` is `air` (a roof conductor), `down` or `earth`; by default vertical runs are down conductors, runs below the level earth conductors and the others roof conductors. Conductors join each other where their ends meet. A project without a conductor type gets one (8 mm, copper); without a lightning protection system, one is made.

| Parameter | Type | | Description |
|---|---|---|---|
| `start` | [x, y] | required | Start in plan. |
| `end` | [x, y] | optional | End in plan (default: the start, for a down conductor). |
| `end_offset` | number | optional | Elevation of the end above the level, meters. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `offset` | number | optional | Elevation of the start above the level, meters. |
| `role` | string | optional | `air`, `down` or `earth` (default: by where it runs). |
| `system` | id or name | optional | Lightning protection system id, name or code (default: the first). |
| `type` | id or name | optional | Conductor type id or name (default: the first conductor type). |

### `lps_conductor_type_create`

*edit* — Create a lightning protection conductor type: a round conductor of `diameter` (meters, 2 to 50 mm; default 8 mm, 3/8" in imperial projects) in `material` (default: copper, else the first metal), named by its diameter unless `name` is given.

| Parameter | Type | | Description |
|---|---|---|---|
| `diameter` | number | optional | Diameter, meters (default 0.008; 3/8" in imperial projects). |
| `material` | string | optional | Material name. |
| `name` | string | optional | Type name. |

### `earth_ring_create`

*edit* — Create an earth ring: buried conductors (role `earth`) around `points` (a closed outline), or by default around the walls of the level, `distance` (default 1 m) out from their outer faces, `depth` (default 0.5 m) below the level, one conductor per side.

| Parameter | Type | | Description |
|---|---|---|---|
| `depth` | number | optional | Depth below the level, meters. |
| `distance` | number | optional | Distance out from the walls' outer faces, meters (without `points`). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `points` | [[x, y], …] | optional | The outline in plan (default: around the walls of the level). |
| `system` | id or name | optional | Lightning protection system id, name or code (default: the first). |
| `type` | id or name | optional | Conductor type id or name (default: the first conductor type). |

### `air_terminal_place`

*edit* — Place an air terminal (a lightning rod) at `position`: on the roof (or floor) surface under it by default, else at the level above; `height` is the rod's height (default: its type's, 1 m for the built-in type). A project without an air terminal type gets one.

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Position in plan. |
| `height` | number | optional | Rod height, meters. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `offset` | number | optional | Elevation of its base above the level, meters (default: the roof under it). |
| `system` | id or name | optional | Lightning protection system id, name or code (default: the first). |
| `type` | id or name | optional | Air terminal type id or name. |

### `earth_electrode_place`

*edit* — Place an earth electrode (a driven rod) at `position`: its top `depth` (default 0.5 m) below the level, `length` long (default: its type's, 2.4 m for the built-in type). A project without an earth electrode type gets one.

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Position in plan. |
| `depth` | number | optional | Depth of its top below the level, meters. |
| `length` | number | optional | Rod length, meters. |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `system` | id or name | optional | Lightning protection system id, name or code (default: the first). |
| `type` | id or name | optional | Earth electrode type id or name. |

### `lps_check`

*query* — Check how the air termination (air terminals and roof conductors) covers the roofs, for a protection `class` (`I` to `IV`, default `III`): the exposed upper faces of roofs (and of floors above the lowest level that nothing covers) are sampled about `spacing` meters apart (default 1 m; more on large roofs) and each point is protected when one air termination point protects it by itself, by the rolling sphere method (sphere radius 20, 30, 45 or 60 m by class) over a horizontal plane at the roof's lowest exposed point, or, on flat parts, when it lies within half the class's mesh width (5, 10, 15 or 20 m) of a roof conductor. Mutual protection between terminals and the shielding by the building itself are left out, which errs on the safe side. Returns the counts, the share protected and up to 200 unprotected points.

| Parameter | Type | | Description |
|---|---|---|---|
| `class` | string | optional | Protection class, `I` to `IV`. |
| `spacing` | number | optional | Distance between the sampled points, meters. |

### `rack_place`

*edit* — Place a communications rack at `position` (on the wall, `offset` above the level, default 1.40 m, 4'-6" in imperial projects) and create the data system its outlets are cabled to. The rack's mark and the system's code are `mark` (default the next free `R1`, `R2`…). A project without a rack type gets one.

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Insertion point in plan (the back of the rack). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `mark` | string | optional | Mark of the rack and code of its cabling. |
| `offset` | number | optional | Elevation above the level, meters. |
| `rotation` | number | optional | Rotation in plan, degrees. |
| `type` | id or name | optional | Rack type id or name. |

### `data_outlet_place`

*edit* — Place a data (or voice) outlet at `position` (on the wall, `offset` above the level, default 0.30 m, 1'-6" in imperial projects), cabled to `rack` (a rack element: its data system) or to the data system `system`; by default to the project's only rack, if it has one. `ports` picks a type with that many jacks (one cable each), made when the project has none; a project without a data outlet type gets one (two jacks).

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Insertion point in plan (the back of the outlet). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `offset` | number | optional | Elevation above the level, meters. |
| `ports` | integer | optional | Jacks per outlet (1 to 48). |
| `rack` | integer | optional | Rack element id. |
| `rotation` | number | optional | Rotation in plan, degrees. |
| `system` | id or name | optional | Data system id, name or code. |
| `type` | id or name | optional | Data outlet type id or name. |

### `cabling_list`

*query* — The structured cabling per rack: for each data system, its rack, its outlets with their jacks and the estimated length of each cable (up to the ceiling of the outlet's level, across along the two axes, down to the rack, plus 3 m of slack), the totals, and the cables longer than the 90 m of a permanent link. `rack` limits it to one rack.

| Parameter | Type | | Description |
|---|---|---|---|
| `rack` | integer | optional | Rack element id. |

### `systems_list`

*query* — List the plumbing systems and electrical circuits with what is in them: element count, pipe or conduit length, fixture units and supply weights of plumbing fixtures, and the devices and load of circuits.

No parameters.

### `system_create`

*edit* — Create a plumbing system, an electrical circuit, the structured cabling of a rack or a lightning protection system. `kind` is `cold_water`, `hot_water`, `sanitary`, `vent`, `rainwater`, `power`, `lighting`, `data` or `lightning`; circuits may name the board (`panel`, a component element; default: the project's only board, if it has one) feeding them and their voltage (default 127 V), and cabling the rack (`panel`; default: the only rack) its outlets are cabled to. The code of a circuit or of cabling defaults to the next free number on its board or rack.

| Parameter | Type | | Description |
|---|---|---|---|
| `kind` | string | required | What it carries. |
| `code` | string | optional | Short code shown in tags (e.g. "AF", a circuit number). |
| `color` | [id, …] | optional | Display color `[r, g, b]` (default: by kind). |
| `name` | string | optional | Name (default: the code, or the kind's name). |
| `panel` | integer | optional | Circuits: the distribution board feeding it (element id; default: the only board). |
| `voltage` | number | optional | Circuits: nominal voltage, volts. |

### `system_set`

*edit* — Change a system or circuit: `params` is a merge patch over the values `systems_list` shows (`name`, `code`, `kind`, `color`, `panel`, `voltage`).

| Parameter | Type | | Description |
|---|---|---|---|
| `params` | JSON | required | Values to change. |
| `system` | id or name | required | System id, name or code. |

### `system_delete`

*edit* — Delete a system or circuit; its pipes, conduits and devices stay, outside any system.

| Parameter | Type | | Description |
|---|---|---|---|
| `system` | id or name | required | System id, name or code. |

### `system_assign`

*edit* — Put elements in a system or circuit, or take them out (`remove`). A pipe or a lightning conductor belongs to one system (it moves); a conduit or cable tray carries several circuits and cabling systems (one is added); a fixture or device connects to one (a data outlet to the cabling of a rack).

| Parameter | Type | | Description |
|---|---|---|---|
| `elements` | [id, …] | required | Pipe, conduit and component ids. |
| `system` | id or name | required | System id, name or code. |
| `remove` | boolean | optional | Take them out instead. |

### `circuit_loads`

*query* — The load schedule of the circuits (of one board, or all): per circuit its devices, installed load (VA) and current at its voltage, with the total per board.

| Parameter | Type | | Description |
|---|---|---|---|
| `panel` | integer | optional | Distribution board element id (default: every circuit). |

### `mep_fittings`

*query* — The fittings where pipes and conduits meet, counted by category, kind, angle (elbows), diameters and system: the fittings takeoff.

| Parameter | Type | | Description |
|---|---|---|---|
| `level` | id or name | optional | Level id or name (default: every level). |

### `mep_connect`

*edit* — Connect plumbing fixtures and electrical devices to the nearest pipe or conduit of their level (or riser of another floor passing through it). A fixture gets a branch to the nearest sanitary drain (falling towards it, with a drop from the floor), and to the nearest cold (and, when it takes it, hot) water pipe; a device gets a conduit to the nearest conduit, with a drop from its mounting height, carrying its circuit. A component with connectors gets a branch from each connector to the nearest run of a system of the connector's kind, from the connector's position and of its diameter (leaving vertically or horizontally as the connector points). Branches end on the runs, which forms tees. `system` connects only to that system or circuit.

| Parameter | Type | | Description |
|---|---|---|---|
| `elements` | [id, …] | required | Fixture and device ids. |
| `system` | id or name | optional | Only to this system or circuit (id, name or code). |

### `mep_sizing`

*query* — Size the building services from their networks (simplified tables of common Brazilian practice): drains by the fixture units upstream, water pipes by the flow from the fixtures' weights (at most 3 m/s), circuits by their current (breaker, copper conductor) and the voltage drop to the farthest device (at most 4 %). Diameters in mm, lengths in m.

No parameters.

### `mep_size_apply`

*edit* — Give each sized pipe the smallest pipe type of the project at least as wide as `mep_sizing` requires. Pipes already wide enough keep their type unless `downsize` is set.

| Parameter | Type | | Description |
|---|---|---|---|
| `downsize` | boolean | optional | Also narrow pipes wider than required. |

## Modifying elements

### `elements_delete`

*edit* — Delete elements (doors and windows of deleted walls go too, piles with their caps; pinned elements stay, and so do the piles of a cap that is not deleted).

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |

### `elements_move`

*edit* — Move elements by `delta` (walls take their doors and windows; doors and windows alone slide along their wall).

| Parameter | Type | | Description |
|---|---|---|---|
| `delta` | [x, y] | required | Displacement `[dx, dy]` in meters. |
| `ids` | [id, …] | required | Element ids. |

### `elements_copy`

*edit* — Copy elements by `delta`; with `count` > 1, a linear array of that many copies spaced by `delta`.

| Parameter | Type | | Description |
|---|---|---|---|
| `delta` | [x, y] | required | Displacement `[dx, dy]` of the first copy, meters (the next ones repeat it). |
| `ids` | [id, …] | required | Element ids. |
| `count` | integer | optional | Number of copies (default 1, at most 1000). |

### `elements_rotate`

*edit* — Rotate elements by `angle` degrees (counter-clockwise) about `center` (default: the center of their extents); with `copy`, rotated copies.

| Parameter | Type | | Description |
|---|---|---|---|
| `angle` | number | required | Angle in degrees, counter-clockwise. |
| `ids` | [id, …] | required | Element ids. |
| `center` | [x, y] | optional | Center of rotation (default: the center of the elements' extents). |
| `copy` | boolean | optional | Rotate copies, keeping the originals. |

### `elements_mirror`

*edit* — Mirror elements across an axis: two points, or the line of a wall, grid, beam or railing (`axis_element`). Copies by default (`copy: false` moves them).

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |
| `axis` | [[x, y], [x, y]] | optional | Two points of the axis. |
| `axis_element` | integer | optional | Id of an element whose line is the axis. |
| `copy` | boolean | optional | Keep the originals and mirror copies (default true). |

### `elements_pin`

*edit* — Pin elements (they cannot be moved or deleted) or unpin them.

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |
| `pinned` | boolean | required | `true` to pin, `false` to unpin. |

### `element_flip`

*edit* — Flip a door or window to the other side of its wall (`facing`) and/or to the other jamb (`hand`).

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Id of the door or window. |
| `facing` | boolean | optional | Flip to the other side of the wall. |
| `hand` | boolean | optional | Flip the hinge to the other jamb. |

### `elements_turn`

*edit* — Turn elements a quarter: components and columns rotate 90° about their insertion point; doors and windows flip to the other side of their wall (the Space key on a selection). Others and pinned elements are left alone.

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |

### `elements_export`

*query* — The elements as a portable set for `elements_paste`: doors and windows of the given walls come along. The set keeps type, level and view ids, so it pastes into this project or one sharing them.

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |

### `elements_paste`

*edit* — Paste a set from `elements_export` moved by `delta`, on `level` when given (else on their own levels). Elements whose type, level, view or wall does not exist here are skipped. Returns the new ids.

| Parameter | Type | | Description |
|---|---|---|---|
| `elements` | JSON | required | The `elements` of an `elements_export` result. |
| `delta` | [x, y] | optional | Displacement `[dx, dy]`, meters (default: none). |
| `level` | id or name | optional | Level id or name to paste on (default: each element's own level). |
| `view` | id or name | optional | View id or name to paste the elements drawn in a view into (detail lines, filled and masking regions, text notes, dimensions, level marks, detail components and imported drawings), at the same drawing coordinates and on its level (default: each one's own view). Every such element must be one the view takes. Elements of the model ignore it. |

### `wall_join`

*edit* — Allow or disallow joining at one end of a wall or beam. A wall end joins other walls; a beam end stops at the face of a structural column it lies in, or meets other beams at the same depth (mitered corners, butt joints).

| Parameter | Type | | Description |
|---|---|---|---|
| `end` | string | required | `start` or `end`. |
| `id` | integer | required | Wall or beam id. |
| `joined` | boolean | required | `true` to allow joining at that end, `false` to keep it free. |

### `wall_attach`

*edit* — Attach the tops of walls to a roof, ceiling or floor (`top`), or take them off it (`detach`). Under the element a wall rises or drops to its underside, following its slope (a gable wall ends in a triangle); elsewhere it keeps its own top. The walls follow the element when it changes.

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Wall ids. |
| `detach` | boolean | optional | Take the tops off what they are attached to. |
| `top` | integer | optional | The roof, ceiling or floor to attach the tops to. |

### `geometry_join`

*edit* — Join the geometry of two elements that touch or overlap (walls, floors, ceilings, roofs, columns, beams, stairs, railings, components). Where they overlap, one cuts the other: structural columns and beams cut walls, floors, ceilings and roofs; floors, ceilings and roofs cut walls; walls cut architectural columns; between equals the first cuts the second. The shared volume is counted once, and plans and sections draw two elements cut in the same material as one piece.

| Parameter | Type | | Description |
|---|---|---|---|
| `first` | integer | required | The first element. |
| `second` | integer | required | The second element. |

### `geometry_unjoin`

*edit* — Unjoin the geometry of two joined elements: each keeps its whole body again.

| Parameter | Type | | Description |
|---|---|---|---|
| `first` | integer | required | The first element. |
| `second` | integer | required | The second element. |

### `group_create`

*edit* — Group elements: selecting one of them selects them all, so they move, copy and rotate together. Elements already in a group leave it.

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | The elements (at least two). |
| `name` | string | optional | Group name (default: "Group N"). |

### `group_ungroup`

*edit* — Ungroup: the group goes, its elements stay.

| Parameter | Type | | Description |
|---|---|---|---|
| `group` | integer | required | Group id (see `groups_list`, or `group` in `element_get`). |

### `trim_corner`

*edit* — Trim or extend two linear elements (walls, beams, grids) to meet at a corner; each pick point marks the part of its element to keep. Arc walls meet lines and other arcs where their circles cross them. Two detail lines meet where their lines, circles or ellipses cross.

| Parameter | Type | | Description |
|---|---|---|---|
| `a` | integer | required | First element id. |
| `b` | integer | required | Second element id. |
| `pick_a` | [x, y] | required | A point on the part of `a` to keep. |
| `pick_b` | [x, y] | required | A point on the part of `b` to keep. |

### `trim_extend`

*edit* — Trim or extend a linear element to the line of a reference element (the circle of an arc wall); the pick point marks the part to keep. Arc walls trim and extend along their circle. A detail line trims or extends to another detail line (also a circle or an ellipse) or to the line of a wall, beam or grid.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Element to trim or extend. |
| `pick` | [x, y] | required | A point on the part to keep. |
| `reference` | integer | required | Id of the element whose line is the boundary. |

### `element_split`

*edit* — Split a linear element (wall, beam, grid) or a detail line at a point; returns the new element. A detail circle or whole ellipse splits into halves, from the point and from the one opposite.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Element id. |
| `point` | [x, y] | required | Where to split (projected on the element's line). |

### `element_offset`

*edit* — Copy a linear element parallel at `distance` (positive: to the left of its direction). Detail lines too: an arc's or a circle's copy is concentric (positive: towards the center of a circle); ellipses have no offset.

| Parameter | Type | | Description |
|---|---|---|---|
| `distance` | number | required | Offset distance, meters: positive to the left of start→end, negative to the right. |
| `id` | integer | required | Element id. |

### `element_align`

*edit* — Align an element onto a line of a reference element, in plan: the edge picked on the element (a wall's face, a column's side) moves onto the line picked on the reference (a face or side, or the location line of a grid or of an element picked without a point). When the two are not parallel the element first turns about the picked point of its edge. An arc wall moves onto the circle of a concentric arc wall (its radius changes). Alignment is in plan, so the elements may be on different levels (a column above onto the face of the column below).

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Element to move. |
| `reference` | integer | required | Element whose line it moves onto. |
| `pick` | [x, y] | optional | Plan point `[x, y]` near the edge of `id` that moves onto the reference line (the edge of its plan outline nearest to it). Without it a linear element moves its location line, and other elements their insertion point, onto the line. |
| `reference_pick` | [x, y] | optional | Plan point `[x, y]` near the line of `reference` to align to (the edge of its plan outline nearest to it, such as a wall face). Without it the reference's location line is used, which a column or component does not have. |

### `element_move_end`

*edit* — Move one end of a wall, beam or grid (joined wall ends follow).

| Parameter | Type | | Description |
|---|---|---|---|
| `end` | string | required | `start` or `end`. |
| `id` | integer | required | Element id. |
| `point` | [x, y] | required | New position of that end. |

### `grids_align_ends`

*edit* — Give grids the extents of a reference grid, so their bubbles line up: each grid parallel to the reference (within 1°) has its ends moved along its own line onto the lines through the reference's ends perpendicular to it (each end to the reference end on its side). Grids that are not parallel and pinned grids are left alone.

| Parameter | Type | | Description |
|---|---|---|---|
| `reference` | integer | required | Id of the reference grid. |
| `ends` | string | optional | Which ends move: `both` (default), `start` (the bubble end) or `end`. |
| `grids` | [id, …] | optional | Ids of the grids to align (default: every other grid parallel to the reference). |

### `element_set`

*edit* — Change an element: its level (doors and windows of a wall follow it), type (of the same category), mark, comments, and any of its `params` as `element_get` shows them (merged: only the fields given change, e.g. `{"height": 2.8}` for a wall or `{"shape": {"Gable": {"slope": 0.5, "ridge_deg": 0}}}` for a roof). The edit is refused if it would leave the model invalid.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Element id. |
| `comments` | string | optional | New comments. |
| `level` | id or name | optional | New level id or name. |
| `mark` | string | optional | New mark. |
| `params` | JSON | optional | Fields of the element's parameters to change. |
| `type` | id or name | optional | New type id or name (same category). |

### `elements_set`

*edit* — Change several elements at once, in one undo step: a new level (doors and windows of a wall follow it), the same `params` merged into each (as `element_set` takes them, e.g. `{"base_offset": -1.5}` for columns), and the same mark or comments. Every element must take the change, or none changes.

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |
| `comments` | string | optional | New comments of every element. |
| `level` | id or name | optional | New level id or name. |
| `mark` | string | optional | New mark of every element. |
| `params` | JSON | optional | Fields of the elements' parameters to change. |

### `elements_set_type`

*edit* — Give elements a type (all must be of the type's category), like Match Type.

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |
| `type` | id or name | required | Type id or name. |

## Dimensions and text

### `dimension_create`

*edit* — Place a dimension in a 2D view (points in the view's drawing coordinates: plan X/Y in plans; horizontal position and height in elevations and sections). Its ends follow the elements (or level lines) their points lie on, so it keeps measuring them. Aligned dimensions measure from `a` to `b`, linear ones along a horizontal or vertical `direction`, ordinate ones from the origin `a` to `b` along that direction (a running dimension; see `dimension_ordinate_create`), angular ones the angle at `vertex` between the rays through `a` and `b`, radial and diameter ones the arc centered at `a` through `b`.

| Parameter | Type | | Description |
|---|---|---|---|
| `a` | [x, y] | required | First point (center for radial and diameter). |
| `b` | [x, y] | required | Second point (a point on the arc for radial and diameter). |
| `view` | id or name | required | View id or name. |
| `direction` | string | optional | Linear and ordinate dimensions: `horizontal` or `vertical` (default: the larger extent). |
| `kind` | string | optional | `aligned` (default), `linear`, `ordinate`, `angular`, `radial` or `diameter`. |
| `offset` | number | optional | Distance of the dimension line from `a` (left of a→b positive), or the arc radius for angular ones (default 1). |
| `type` | id or name | optional | Dimension type id or name (default: the default dimension type). |
| `vertex` | [x, y] | optional | Vertex of an angular dimension. |

### `tag_move`

*edit* — Move the tag of an element in a plan (the tag of a door, window, beam, column or footing): by `delta` from where it is, to `offset` from its default spot, or back there with `reset`. A tag moved away from its element gets a leader to it. Each plan keeps its own tag positions.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Id of the element the tag names. |
| `view` | id or name | required | Plan id or name. |
| `delta` | [x, y] | optional | Move by `[dx, dy]` meters in plan. |
| `offset` | [x, y] | optional | Place the tag `[dx, dy]` meters from its default spot. |
| `reset` | boolean | optional | Put the tag back in its default spot. |

### `dimension_chain_create`

*edit* — Place a chain of dimensions on one dimension line (continuous dimensioning): one dimension from each point to the next, all measured along the same direction and drawn on the same line, in one undo step. Points are in the view's drawing coordinates, as for `dimension_create`, and each end follows the element (or level line) it lies on. In an aligned chain, segments along the direction are aligned dimensions and the others measure along that direction.

| Parameter | Type | | Description |
|---|---|---|---|
| `points` | [[x, y], …] | required | The measured points in order (at least 2, at most 500). |
| `view` | id or name | required | View id or name. |
| `angle` | number | optional | Direction to measure along, degrees counter-clockwise from the X axis (overrides `direction` and the direction of the first two points). |
| `direction` | string | optional | Linear chains: `horizontal` or `vertical` (default: the larger extent of the first two points). |
| `kind` | string | optional | `aligned` (default: along the first two points) or `linear` (horizontal or vertical). |
| `offset` | number | optional | Distance of the dimension line from the first point, positive to the left of the direction (default 1). |
| `type` | id or name | optional | Dimension type id or name (default: the default dimension type). |

### `dimension_ordinate_create`

*edit* — Place running (ordinate) dimensions on one dimension line: one dimension from the origin (the first point) to each other point, measured along the same direction, each showing its running total from the origin, in one undo step. Points are in the view's drawing coordinates, as for `dimension_create`, and each end follows the element (or level line) it lies on, so the totals follow the model; giving one a new value with `dimension_set_value` moves what its point measures, keeping the origin.

| Parameter | Type | | Description |
|---|---|---|---|
| `points` | [[x, y], …] | required | The origin, then the measured points (at least 2 in all, at most 500). |
| `view` | id or name | required | View id or name. |
| `angle` | number | optional | Direction to measure along, degrees counter-clockwise from the X axis (overrides `direction`). |
| `direction` | string | optional | `horizontal` or `vertical` (default: along the origin and the second point). |
| `offset` | number | optional | Distance of the dimension line from the origin, positive to the left of the direction (default 1). |
| `type` | id or name | optional | Dimension type id or name (default: the default dimension type). |

### `dimension_set_value`

*edit* — Give a dimension a new value (meters; degrees for angular dimensions): in plans what its moving end measures moves or stretches; between level lines in elevations and sections the upper level moves. Radial and diameter dimensions are read-only.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Dimension element id. |
| `value` | number | required | New value: meters, or degrees for angular dimensions. |

### `text_create`

*edit* — Place a text note in a 2D view (points in the view's drawing coordinates, as for dimensions). Without `width` it is single-line text (only line breaks in `text` break it); with `width` (paper millimeters) it is multiline text wrapped to that width. Leaders point from the text to what it describes, each with one segment or two (with an `elbow`).

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Top left corner of the text. |
| `text` | string | required | The text; a line break starts a new line. |
| `view` | id or name | required | View id or name. |
| `leaders` | [object, …] | optional | Leaders from the text. |
| `rotation` | number | optional | Rotation in degrees counter-clockwise. |
| `type` | id or name | optional | Text type id or name (default: the default text type). |
| `width` | number | optional | Wrapping width in paper millimeters (multiline text). |

### `detail_line_create`

*edit* — Draw detail lines in a 2D view (plan, ceiling plan, elevation, section, legend or drafting view): lines, arcs, circles and ellipses in the view's drawing coordinates (meters, as for dimensions and text notes), shown only in that view, on top of the model. Each curve becomes one element (a polyline is its segments; rectangles and polygons their sides), all in one undo step. Detail lines are left out of 3D views and IFC export; DXF export puts them on a layer named after their style.

| Parameter | Type | | Description |
|---|---|---|---|
| `curves` | [JSON, …] | required | The curves to draw, as for sketches (`line`, `arc`, `circle`, `ellipse`, `rectangle`, `polygon`). |
| `view` | id or name | required | View id or name. |
| `style` | string | optional | Line style: thin (default), medium, wide, hidden, dashed, center or dotted, or the name of one of the project's line styles (`line_styles_list`). |

### `detail_component_create`

*edit* — Make a detail component (a reusable 2D component for construction details) as a new type named `name`, in one undo step: from `pieces` (meters in its frame), or from `elements` of `view` (detail lines, filled and masking regions, text notes and other detail components), whose point `origin` (default: the lower left corner of their extents) becomes the component's origin. With `replace` (the default when elements are given) those elements are deleted and the new component is placed where they were. Detail lines keep their pen, regions their fill (solid or pattern; openings as outlines), text notes their text at its printed height, and detail components become nested pieces.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name of the component and of its type. |
| `description` | string | optional | Description of the type. |
| `elements` | [id, …] | optional | Ids of the view's elements to make the component from. |
| `origin` | [x, y] | optional | The point of the view that becomes the component's origin. |
| `pieces` | [JSON, …] | optional | Pieces, as `component_sheet_piece_add` takes them (meters). |
| `replace` | boolean | optional | Replace `elements` by the new component (default true). |
| `view` | id or name | optional | View id or name of `elements`. |

### `detail_component_place`

*edit* — Place a detail component in a 2D view (plan, ceiling plan, elevation, section, legend or drafting view): its origin at `position` (view coordinates, meters), turned `rotation` degrees, sized by `scale` and flipped across its own X axis first when `mirrored`. It is drawn only in that view, over the model and its regions, and goes with the view.

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | Where its origin goes, [x, y] in meters. |
| `type` | id or name | required | A detail component type: id or name. |
| `view` | id or name | required | View id or name. |
| `mirrored` | boolean | optional | Flipped across its own X axis before turning (default false). |
| `rotation` | number | optional | Degrees counter-clockwise (default 0). |
| `scale` | number | optional | Size factor (default 1). |

### `browser_schemes_list`

*query* — The project browser's organization schemes: each with its name and the criteria it groups the views by, outermost first, and whether it is in use. `in_use` is null while the browser lists the views by type alone (the default), and `criteria` are the names `group_by` takes.

No parameters.

### `browser_scheme_create`

*edit* — Create a project browser organization scheme: a named way to group the browser's views. `group_by` lists the criteria from the outermost group in (`view_type`, `level`, `discipline`, `phase`); a view without a value for one (a section has no level) stays beside that criterion's groups. The new scheme is put in use unless `use_it` is false.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name (unique among the schemes). |
| `group_by` | [string, …] | optional | Criteria, outermost first: `view_type`, `level`, `discipline`, `phase` (default: `["view_type", "level"]`). |
| `use_it` | boolean | optional | Make the browser follow the new scheme (default true). |

### `browser_scheme_set`

*edit* — Change a project browser organization scheme: its name or the criteria it groups the views by. The browser follows at once when it is in use.

| Parameter | Type | | Description |
|---|---|---|---|
| `scheme` | id or name | required | Scheme id or name. |
| `group_by` | [string, …] | optional | New criteria, outermost first: `view_type`, `level`, `discipline`, `phase`. |
| `name` | string | optional | New name. |

### `browser_scheme_delete`

*edit* — Delete a project browser organization scheme. When it was in use the browser goes back to listing the views by type.

| Parameter | Type | | Description |
|---|---|---|---|
| `scheme` | id or name | required | Scheme id or name. |

### `browser_scheme_use`

*edit* — Choose the scheme the project browser follows. Without `scheme` (or with null) the browser lists the views by type alone, the default.

| Parameter | Type | | Description |
|---|---|---|---|
| `scheme` | id or name | optional | Scheme id or name; omitted or null for the default list by view type. |

### `browser_tree`

*query* — The project browser's tree: the views grouped as the scheme in use (or `scheme`, or the criteria in `group_by`) says. A group row has `group` (its criterion), `name`, `value` (the view type, level id, discipline or phase id), how many `views` it holds and its `children`; a view row has `view` (its id) and `name`. Plans come in the order of their levels.

| Parameter | Type | | Description |
|---|---|---|---|
| `group_by` | [string, …] | optional | Criteria to group by instead of a scheme's, outermost first: `view_type`, `level`, `discipline`, `phase`. |
| `scheme` | id or name | optional | Scheme id or name (default: the scheme in use). |

### `line_styles_list`

*query* — The line styles detail lines and region boundaries can use: the built-in ones (by code) and the project's (by name), each with its weight on paper (mm), color, dash pattern and how many elements use it.

No parameters.

### `line_style_create`

*edit* — Create a project line style for detail lines and region boundaries: a unique name (not a built-in code), a weight on paper, a color and a dash pattern. Commands then name it like a built-in style.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name (unique, not a built-in style's code). |
| `color` | [id, …] | optional | Color `[r, g, b]`, 0 to 255 (default black). |
| `pattern` | string | optional | Dash pattern: solid (default), dash, short_dash, dot or dash_dot. |
| `weight` | number | optional | Width on paper, millimeters (0.05 to 5; default 0.25). |

### `line_style_set`

*edit* — Change a project line style: name, weight, color or pattern. The lines and boundaries drawn with it follow.

| Parameter | Type | | Description |
|---|---|---|---|
| `style` | id or name | required | Line style id or name. |
| `color` | [id, …] | optional | Color `[r, g, b]`. |
| `name` | string | optional | New name. |
| `pattern` | string | optional | Dash pattern: solid, dash, short_dash, dot or dash_dot. |
| `weight` | number | optional | Width on paper, millimeters (0.05 to 5). |

### `line_style_delete`

*edit* — Delete a project line style. The detail lines and region boundaries drawn with it move to `replace_with` (a built-in code or another project style's name; default `thin`) in the same undo step.

| Parameter | Type | | Description |
|---|---|---|---|
| `style` | id or name | required | Line style id or name. |
| `replace_with` | string | optional | The style that takes its place where it is used (default `thin`). |

### `filled_region_create`

*edit* — Draw a filled region in a 2D view (plan, ceiling plan, elevation, section, legend or drafting view): the closed loops of `curves` in the view's drawing coordinates (meters), the largest the outline and those inside it openings, filled as its type says (a pattern over a background, or a solid color) and edged in a line style. It is drawn only in that view, over the model and under the view's annotations. A project without a filled region type gets one (diagonal lines over white).

| Parameter | Type | | Description |
|---|---|---|---|
| `curves` | [JSON, …] | required | The curves, closed into loops, as for sketches (`line`, `arc`, `circle`, `ellipse`, `rectangle`, `polygon`). |
| `view` | id or name | required | View id or name. |
| `boundary` | string | optional | Boundary line style (a built-in code or a project style's name; default `thin`), or `none` for no boundary. |
| `type` | id or name | optional | Filled region type id or name (default: the project's first). |

### `masking_region_create`

*edit* — Draw a masking region in a 2D view: an opaque white area (the closed loops of `curves`, openings inside the outline) that hides the model's linework of the view under it, while the view's annotations (detail lines, dimensions, texts, tags) stay on top. Drawn only in that view.

| Parameter | Type | | Description |
|---|---|---|---|
| `curves` | [JSON, …] | required | The curves, closed into loops, as for sketches. |
| `view` | id or name | required | View id or name. |
| `boundary` | string | optional | Boundary line style (a built-in code or a project style's name), or `none` (the default) for no boundary. |

### `filled_region_type_create`

*edit* — Create a filled region type: a foreground pattern over a background pattern, each with its color (a `solid` pattern fills with its color; a `none` background leaves the model visible between the pattern lines). Change it later with `type_set` (`{"fill": {...}}`).

| Parameter | Type | | Description |
|---|---|---|---|
| `background` | string | optional | Background pattern (default `solid`), as for `foreground`. |
| `background_color` | [id, …] | optional | Background color `[r, g, b]` (default white). |
| `foreground` | string | optional | Foreground pattern (default `diagonal`): none, solid, diagonal, diagonal_back, crosshatch, horizontal, vertical, grid, brick, concrete, sand, insulation or earth. |
| `foreground_color` | [id, …] | optional | Foreground color `[r, g, b]` (default black). |
| `name` | string | optional | Type name (default: the template's filled region name). |

### `level_mark_create`

*edit* — Place a level mark in a plan, an elevation or a section: the finished level of a point above a shelf line and its raw (structural) level under it, each after the type's datum (the value written for the project's zero) is added. Plans draw a circle with two filled quarters on the point and measure the top of the highest floor of the plan's level under it (the level where there is none), with the top of that floor's structure as the raw level; elevations and sections draw a triangle pointing down to the point and measure its height (`position` y), with the raw level of a floor whose top the point lies on, in sections. `element_get` gives the levels and texts (`derived`); `element_set` changes the mark's `params`.

| Parameter | Type | | Description |
|---|---|---|---|
| `position` | [x, y] | required | The point measured, in the view's drawing coordinates (in elevations and sections `y` is the elevation). |
| `view` | id or name | required | View id or name: a floor or ceiling plan, an elevation or a section. |
| `finished` | number | optional | Finished level to write instead of the measured one, meters (an elevation of the project, before the datum). |
| `flip` | boolean | optional | Write the values on the left of the symbol (default: on the right). |
| `raw` | number | optional | Raw level to write instead of the measured one, meters. |
| `show_raw` | boolean | optional | Write the raw level when there is one (default true). |
| `type` | id or name | optional | Level mark type id or name (default: the default level mark type). |

### `level_mark_type_create`

*edit* — Create a level mark type: the text size, the datum (the value written for the project's zero, e.g. 100 when the ground floor is +100.00) and the suffixes after the finished and raw levels. A project gets one with its first level mark; `type_set` changes them.

| Parameter | Type | | Description |
|---|---|---|---|
| `datum` | number | optional | Value written for the project's zero, meters (default 0). |
| `finished_suffix` | string | optional | Text after the finished level (default: the language's, e.g. "(N.A.)"). |
| `name` | string | optional | Type name (default: the language's). |
| `raw_suffix` | string | optional | Text after the raw level (default: the language's, e.g. "(N.O.)"). |
| `text_size` | number | optional | Text size, paper millimeters (default 2.5; 3/32", 2.38125, in imperial projects). |

### `text_add_leader`

*edit* — Add a leader to a text note.

| Parameter | Type | | Description |
|---|---|---|---|
| `end` | [x, y] | required | The point the leader points at, where its symbol is drawn. |
| `id` | integer | required | Text element id. |
| `elbow` | [x, y] | optional | A bend point that splits the leader in two segments (text → elbow → end). |
| `symbol` | string | optional | End symbol: `Arrow`, `OpenArrow`, `Triangle`, `Dot`, `Circle`, `Square`, `FilledSquare`, `Tick` or `None` (default: the text type's symbol). |

### `text_remove_leader`

*edit* — Remove a leader from a text note.

| Parameter | Type | | Description |
|---|---|---|---|
| `id` | integer | required | Text element id. |
| `index` | integer | required | Index of the leader (from 0, as in `params.leaders`). |

## Types, components, materials

### `type_set`

*edit* — Change a type: its name, its parameters (`params`, merged into what `type_get` shows: layers, sizes, materials, a component's `def`…) and its identity data (`info`: type mark, description, manufacturer, cost…). A column or beam type takes `profile` (a structural profile's name or id; null is refused: these types always point to a profile), `profile_rotation_deg` and `profile_mirror`. `width`/`height` are the box of the turned profile; changing them alone re-points the type to a profile of that section, unturned and unmirrored. Changing a component's definition updates every type of that component, each keeping its own parameter values.

| Parameter | Type | | Description |
|---|---|---|---|
| `type` | id or name | required | A level, type, view or material: its numeric id or its name. |
| `info` | JSON | optional | Fields of the identity data to change, shaped like `info` in `type_get`. |
| `name` | string | optional | New name (unique in its category). |
| `params` | JSON | optional | Fields of the type's parameters to change, shaped like `params` in `type_get`. |
| `renames` | [[old, new], …] | optional | Component parameters renamed by a new `def`, as `[old, new]` pairs in order, so the component's other types keep their values. |

### `type_duplicate`

*edit* — Copy a type under a new name (default "<name> (copy)"); the given elements of the type switch to the copy.

| Parameter | Type | | Description |
|---|---|---|---|
| `type` | id or name | required | A level, type, view or material: its numeric id or its name. |
| `elements` | [id, …] | optional | Elements of the type to switch to the copy. |
| `name` | string | optional | Name of the copy (default: "<name> (copy)"). |

### `type_delete`

*edit* — Delete a type that no element uses (the last type of a category is kept, except a title block: sheets on a deleted title block move to the first other one, or to none).

| Parameter | Type | | Description |
|---|---|---|---|
| `type` | id or name | required | A level, type, view or material: its numeric id or its name. |

### `component_create`

*edit* — Create a component type: `def` holds its parameters and pieces, shaped like a component type's `params.def` in `type_get`. A piece's `shape` is a `Box` {x, y, z, width, depth, height}, a `Cylinder` {x, y, z, radius, height}, an `Extrusion` {points: [[x, y], …], z, height} (a plan outline extruded up), a `Profile` {plane: `Plan`, `Front` or `Side`, points: [{u, v, radius}, …], offset, depth} (lines and arcs drawn on a plane: u, v are x, y in plan, x, z from the front and y, z from the side; the plane stands at `offset` on the remaining axis and extrudes `depth` along it; a point's optional `radius` makes the segment to the next point an arc, positive bulging outwards) or a `Revolution` {plane, points, offset, axis, angle} (the profile turned about the line u = axis of its plane). Every field is an expression over the parameters (meters, degrees). A piece with `void: true` is cut out of the solid pieces it overlaps; one with `plan_only: true` is a plan symbol (a door's swing), drawn in plans and not a solid. A `kind` of `Door` or `Window` with an `opening` {width, height, sill, own_symbol, operation, door_operation} (expressions, then a window's `operation`: `Fixed`, `SideHungLeft`, `SideHungRight` or `DoubleSideHung`, drawn as swings in plans and written as IFC panel properties, and a door's `door_operation`: `SingleSwingLeft` (default), `SingleSwingRight` or `DoubleDoor`, one swing per leaf in plans and the IFC operation type; `door_type_create` makes the built-in ones) makes a door or window type, placed with `door_create` or `window_create`: it cuts that opening in its wall; its frame has x along the wall from the opening's center, y across it (towards the side a door or window opens to), z up from the sill, and `WallThickness` is the wall's thickness. `connectors` [{name, system: `ColdWater`, `HotWater`, `Sanitary`, `Vent`, `Rainwater`, `Power` or `Lighting`, x, y, z, direction: `Down`, `Up`, `Front` (-y), `Back` (+y), `Left` (-x) or `Right` (+x), diameter}] are where `mep_connect` joins pipes and conduits (a toilet's inlet and outlet). With a `kind` in place of `def` the component starts from that kind's own parameters and pieces, to be shaped in the component environment or with the `component_*` commands.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name of the component and of its first type. |
| `def` | JSON | optional | The definition: `params` (name, value, kind, instance) and `items` (pieces), shaped like a component type's `params.def` in `type_get`. Left out, the component starts from its `kind`. |
| `kind` | string | optional | The kind a component made without a `def` starts from: `Furniture`, `Casework`, `Plumbing` and `Electrical` (a box sized by width, depth and height parameters), `Lighting` (a cylinder sized by diameter and height), `Door` (width and height) and `Window` (width, height and sill), which cut their opening in a wall, `Planting` (an empty tree or plant component to shape), or `Generic` (the default: a fixed box and no parameters, which are yours to add). Not given together with `def`, whose own `kind` says it. |

### `component_get`

*query* — The component of a type, as the modeling environment shows it: the definition, the value of every parameter and reference plane, the value of each dimension, where each connector is, the problems found, and the component's types with their parameter values. A title block's pictures show as `format`, `width`, `height` (pixels) and `bytes`, not their bytes.

| Parameter | Type | | Description |
|---|---|---|---|
| `type` | id or name | required | A type of the component: id or name. |

### `component_param_add`

*edit* — Add a parameter to a component (to every type, with the same value).

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name used in expressions (letters, digits and `_`, not starting with a digit). |
| `type` | id or name | required | A type of the component: id or name. |
| `instance` | boolean | optional | Elements may override the value (sheets, for a title block's text parameter). |
| `kind` | string | optional | `length` (default), `angle`, `number` or `text` (a title block's per-sheet field, with `instance`). |
| `text` | string | optional | Value of a `text` parameter (default empty). |
| `value` | number | optional | Value (meters for lengths, degrees for angles; default 0). |

### `component_param_set`

*edit* — Change a component parameter: its value (for the type named only), kind, whether elements may override it, or its name (every expression, dimension and type follows a rename).

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | The parameter's current name. |
| `type` | id or name | required | A type of the component: id or name (the value changes for this type). |
| `instance` | boolean | optional | Elements may override the value. |
| `kind` | string | optional | `length`, `angle` or `number`. |
| `rename` | string | optional | New name. |
| `value` | number | optional | New value for the type named. |

### `component_param_delete`

*edit* — Remove a component parameter; refused while an expression reads it. Dimensions it labels lose the label (their planes keep their place).

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | The parameter's name. |
| `type` | id or name | required | A type of the component: id or name. |

### `component_ref_plane_add`

*edit* — Add a reference plane to a component: square to `axis` (`x`: a left/right plane, `y`: front/back, `z`: a height), at the position `at`, an expression that may read parameters and other planes (`Left + Width`).

| Parameter | Type | | Description |
|---|---|---|---|
| `at` | string | required | Position along the axis (meters): a number or an expression. |
| `axis` | string | required | `x`, `y` or `z`. |
| `type` | id or name | required | A type of the component: id or name. |
| `name` | string | optional | Name (default `Plane1`, `Plane2`…). |

### `component_ref_plane_set`

*edit* — Move or rename a reference plane: `at` is its new position (an expression); a rename follows into every expression and dimension.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | The plane's current name. |
| `type` | id or name | required | A type of the component: id or name. |
| `at` | string | optional | New position: a number or an expression. |
| `rename` | string | optional | New name. |

### `component_ref_plane_delete`

*edit* — Remove a reference plane and the dimensions on it; refused while an expression reads it.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | The plane's name. |
| `type` | id or name | required | A type of the component: id or name. |

### `component_ref_line_add`

*edit* — Add a reference line on one of the sketch planes (`plan`, `front` or `side`, standing at `offset` along its normal axis): its ends are `[u, v]` pairs of expressions in that plane.

| Parameter | Type | | Description |
|---|---|---|---|
| `end` | [old, new] | required | Second end `[u, v]`. |
| `plane` | string | required | `plan`, `front` or `side`. |
| `start` | [old, new] | required | First end `[u, v]`: numbers or expressions. |
| `type` | id or name | required | A type of the component: id or name. |
| `name` | string | optional | Name (default `Line1`, `Line2`…). |
| `offset` | string | optional | Position of the sketch plane along its normal axis (default 0). |

### `component_ref_line_delete`

*edit* — Remove a reference line.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | The line's name. |
| `type` | id or name | required | A type of the component: id or name. |

### `component_dimension_add`

*edit* — Dimension two parallel reference planes in a view (`plan`, `front` or `side`, one that sees them edge-on). With `label`, a length parameter drives it: the second plane moves to the first plus the parameter (the parameter is created with the current distance when missing).

| Parameter | Type | | Description |
|---|---|---|---|
| `from` | string | required | The plane it measures from. |
| `to` | string | required | The plane it measures to (the one a label moves). |
| `type` | id or name | required | A type of the component: id or name. |
| `view` | string | required | `plan`, `front` or `side`. |
| `label` | string | optional | Parameter that drives the distance. |
| `place` | number | optional | Where the dimension line sits across the planes (meters, default 0). |

### `component_dimension_set`

*edit* — Change a dimension between reference planes: label it with a parameter (`label`, created when missing), take its label off (`label: ""`, the planes stay), move its line (`place`) or give it a new `value` (the label's parameter for this type, or the moving plane's position).

| Parameter | Type | | Description |
|---|---|---|---|
| `index` | integer | required | The dimension's index (from `component_get`). |
| `type` | id or name | required | A type of the component: id or name. |
| `label` | string | optional | Parameter that drives it; empty to take the label off. |
| `place` | number | optional | Where the dimension line sits across the planes. |
| `value` | number | optional | New distance (meters). |

### `component_dimension_delete`

*edit* — Remove a dimension between reference planes (the planes stay).

| Parameter | Type | | Description |
|---|---|---|---|
| `index` | integer | required | The dimension's index. |
| `type` | id or name | required | A type of the component: id or name. |

### `component_equal_add`

*edit* — Keep the plane `middle` halfway between the parallel planes `a` and `b` (an equality, EQ): its position becomes `(a + b) / 2`, so it follows whatever moves them. Shown in `view` (`plan`, `front` or `side`) as two equal dimensions.

| Parameter | Type | | Description |
|---|---|---|---|
| `a` | string | required | One outer plane. |
| `b` | string | required | The other outer plane. |
| `middle` | string | required | The plane kept halfway (not an origin plane). |
| `type` | id or name | required | A type of the component: id or name. |
| `view` | string | required | `plan`, `front` or `side`. |
| `place` | number | optional | Where the dimensions' line sits across the planes (meters, default 0). |

### `component_equal_delete`

*edit* — Remove an equality (EQ); its middle plane stays halfway as written.

| Parameter | Type | | Description |
|---|---|---|---|
| `index` | integer | required | The equality's index (from `component_get`). |
| `type` | id or name | required | A type of the component: id or name. |

### `component_piece_add`

*edit* — Add a piece drawn as a sketch on the `plan`, `front` or `side` plane: an `extrusion` of the closed outline from `start` to `end` along the plane's normal (Z for the plan, Y for the front, X for the side); a `revolution` about the line u = `axis` of the plane, `angle` degrees (default 360); a `sweep` of the closed outline (`points`: u to the left of the path, v along the normal) along the open `path` drawn on the plane at `start`, mitered at its corners; or a `blend` from the outline at `start` to the `top` outline at `end`. Coordinates are numbers or expressions; a reference plane's name locks a coordinate (or `start`, `end`, `axis`) to that plane, so the piece follows it. With `void`, the piece is cut out of the solid pieces it overlaps.

| Parameter | Type | | Description |
|---|---|---|---|
| `plane` | string | required | `plan`, `front` or `side`. |
| `points` | [object, …] | required | The closed outline, in order (a sweep's section, a blend's bottom). |
| `type` | id or name | required | A type of the component: id or name. |
| `angle` | number or string | optional | Revolutions: degrees turned (default 360). |
| `axis` | number or string | optional | Revolutions: the axis, u = `axis` in the sketch plane. |
| `end` | number or string | optional | Extrusions and blends: where they end along the normal (default 0.1 past `start`). |
| `kind` | string | optional | `extrusion` (default), `revolution`, `sweep` or `blend`. |
| `material` | string | optional | Project material name. |
| `name` | string | optional | Name (default `Piece1`, `Piece2`…). |
| `path` | [object, …] | optional | Sweeps: the open path the section follows. |
| `start` | number or string | optional | Where the sketch plane stands along its normal (default 0). |
| `top` | [object, …] | optional | Blends: the closed outline at `end`. |
| `void` | boolean | optional | Cut out of the solid pieces it overlaps. |

### `component_piece_set`

*edit* — Change a piece of a component: its name, material, whether it is a void or drawn in plans, and for sketched pieces their outline, `start`, `end` (extrusions), `axis` and `angle` (revolutions).

| Parameter | Type | | Description |
|---|---|---|---|
| `index` | integer | required | The piece's index (from `component_get`). |
| `type` | id or name | required | A type of the component: id or name. |
| `angle` | number or string | optional | Revolutions: degrees turned. |
| `axis` | number or string | optional | Revolutions: the axis, u = `axis` in the sketch plane. |
| `detail` | [string, …] | optional | The detail levels of the views that show the piece: any of `coarse`, `medium` and `fine` (all three by default). |
| `end` | number or string | optional | Extrusions: where they end along the normal. |
| `in_plan` | boolean | optional | Drawn in plans. |
| `material` | string | optional | Project material name (empty for the default color). |
| `name` | string | optional | New name. |
| `path` | [object, …] | optional | Sweeps: new path. |
| `points` | [object, …] | optional | New outline (a sweep's section, a blend's bottom). |
| `start` | number or string | optional | Where the sketch plane stands along its normal. |
| `top` | [object, …] | optional | Blends: new top outline. |
| `void` | boolean | optional | Cut out of the solid pieces it overlaps. |

### `component_piece_delete`

*edit* — Remove a piece of a component (it may be left with none while it is shaped).

| Parameter | Type | | Description |
|---|---|---|---|
| `index` | integer | required | The piece's index. |
| `type` | id or name | required | A type of the component: id or name. |

### `component_connector_add`

*edit* — Add a connector to a component: where a pipe or conduit of `system` joins it. `x`, `y` and `z` place it in the component's frame (numbers or expressions; a reference plane's name locks it to that plane), and `direction` is the way runs leave it.

| Parameter | Type | | Description |
|---|---|---|---|
| `type` | id or name | required | A type of the component: id or name. |
| `diameter` | number or string | optional | Outside diameter of the run it takes, meters (default: unset). |
| `direction` | string | optional | Which way runs leave it: `down` (default), `up`, `front`, `back`, `left` or `right`. |
| `name` | string | optional | Name (default `Connector1`, `Connector2`…). |
| `system` | string | optional | Kind of system: `cold_water` (default), `hot_water`, `sanitary`, `vent`, `rainwater`, `power` or `lighting`. |
| `x` | number or string | optional | Position across the component (default 0). |
| `y` | number or string | optional | Position front to back (default 0). |
| `z` | number or string | optional | Height (default 0). |

### `component_connector_set`

*edit* — Change a connector of a component: its name, kind of system, position (`x`, `y`, `z`, numbers or expressions; a reference plane's name locks it to that plane), direction or diameter (empty to clear it).

| Parameter | Type | | Description |
|---|---|---|---|
| `index` | integer | required | The connector's index (from `component_get`). |
| `type` | id or name | required | A type of the component: id or name. |
| `diameter` | number or string | optional | Outside diameter of the run it takes, meters (empty to clear it). |
| `direction` | string | optional | Which way runs leave it: `down`, `up`, `front`, `back`, `left` or `right`. |
| `name` | string | optional | New name. |
| `system` | string | optional | Kind of system: `cold_water`, `hot_water`, `sanitary`, `vent`, `rainwater`, `power` or `lighting`. |
| `x` | number or string | optional | Position across the component. |
| `y` | number or string | optional | Position front to back. |
| `z` | number or string | optional | Height. |

### `component_connector_delete`

*edit* — Remove a connector of a component.

| Parameter | Type | | Description |
|---|---|---|---|
| `index` | integer | required | The connector's index. |
| `type` | id or name | required | A type of the component: id or name. |

### `component_script_set`

*edit* — Write the Lua source of a script piece of a component: the piece named, or a new script piece. What the script declares becomes the component's parameters and hotspots (values of those still declared are kept); a script that does not load is refused, one that fails while building its pieces is kept and its error returned.

| Parameter | Type | | Description |
|---|---|---|---|
| `source` | string | required | The Lua source: `param.length { "width", default = 1 }` declarations and a `geometry(p)` function returning pieces (`box`, `cylinder`, `extrusion`, `profile`, `revolution`, `sweep`, `blend`, `mesh`). |
| `type` | id or name | required | A type of the component: id or name. |
| `material` | string | optional | Material of the pieces that set none (project material name). |
| `name` | string | optional | Name of a new piece (default `Script1`, `Script2`…). |
| `piece` | string | optional | The script piece to change; a new piece when left out. |

### `component_params_get`

*query* — The parameters and hotspots of a component type, with the type's values; or of placed components (all of one component), with the value they share or `mixed` where they differ.

| Parameter | Type | | Description |
|---|---|---|---|
| `elements` | [id, …] | optional | Placed components of one component (leave out with `type`). |
| `type` | id or name | optional | A type of the component: id or name (leave out with `elements`). |

### `component_params_set`

*edit* — Set a parameter of a component type (every element of the type follows) or the instance value of placed components (all of one component; the parameter must be an instance one). Values: numbers (meters, degrees), true/false, texts, choice keys or material names. One undo step; when any target refuses the value nothing changes.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | The parameter's name. |
| `value` | JSON | required | The new value. |
| `elements` | [id, …] | optional | Placed components of one component (leave out with `type`). |
| `type` | id or name | optional | A type of the component: id or name (leave out with `elements`). |

### `component_hotspot_move`

*edit* — Move a hotspot of a placed component (what its script reads as `p.<name>`). The point is in the component's frame (meters from the insertion point, x to its right, y to its back, z up) unless `world`.

| Parameter | Type | | Description |
|---|---|---|---|
| `element` | integer | required | The placed component. |
| `name` | string | required | The hotspot's name. |
| `point` | [number, …] | required | The new position `[x, y, z]`. |
| `world` | boolean | optional | The point is in project coordinates (z absolute) instead of the component's frame. |

### `material_create`

*edit* — Create a material (default name "Material"; a number is added when taken).

| Parameter | Type | | Description |
|---|---|---|---|
| `color` | [id, …] | optional | Shading color `[r, g, b]`, 0–255 (default light gray). |
| `name` | string | optional | Name (default: "Material"; a number is added when taken). |

### `material_duplicate`

*edit* — Copy a material under a new name (default "<name> (copy)").

| Parameter | Type | | Description |
|---|---|---|---|
| `material` | id or name | required | A level, type, view or material: its numeric id or its name. |
| `name` | string | optional | Name of the copy (default: "<name> (copy)"). |

### `material_rename`

*edit* — Rename a material; every type using it follows.

| Parameter | Type | | Description |
|---|---|---|---|
| `material` | id or name | required | A level, type, view or material: its numeric id or its name. |
| `name` | string | required | New name. |

### `material_set`

*edit* — Change a material's fields (merged into what `material_get` shows: description, class, colors, patterns, transparency, finish, physical values…). Use `material_rename` to rename it.

| Parameter | Type | | Description |
|---|---|---|---|
| `material` | id or name | required | A level, type, view or material: its numeric id or its name. |
| `values` | JSON | required | Fields to change, shaped like the result of `material_get`. |

### `material_delete`

*edit* — Delete a material no type uses.

| Parameter | Type | | Description |
|---|---|---|---|
| `material` | id or name | required | A level, type, view or material: its numeric id or its name. |

### `material_texture_set`

*edit* — Give a material an image texture (shown on its faces in the textures visual style of 3D views), or change how its texture is laid out. The image comes from `path` or from `data` (base64); with neither, the current texture's size, offset and rotation change. One repeat covers `width` × `height` meters on the faces: on floors and other horizontal faces along plan X and Y, on walls and slopes level across the face and up it.

| Parameter | Type | | Description |
|---|---|---|---|
| `material` | id or name | required | A level, type, view or material: its numeric id or its name. |
| `data` | string | optional | The image file's bytes as base64 (instead of `path`). |
| `height` | number | optional | Height of one repeat, meters (default: the current one, else the width in the image's proportions). |
| `offset` | [x, y] | optional | Where a repeat starts, `[x, y]` meters along the face (default: the current one, else `[0, 0]`). |
| `path` | string | optional | Path of a PNG, JPEG, GIF, BMP or WebP file. |
| `rotation` | number | optional | Rotation of the image on the faces, degrees counter-clockwise (default: the current one, else 0). |
| `width` | number | optional | Width of one repeat, meters (default: the current one, else 1 m). |

### `material_texture_clear`

*edit* — Take a material's texture away (its faces show the appearance color).

| Parameter | Type | | Description |
|---|---|---|---|
| `material` | id or name | required | A level, type, view or material: its numeric id or its name. |

### `fill_patterns_list`

*query* — The fill patterns materials, filled regions and view overrides can use: the built-in ones (by code) and the project's (by name, with their kind, scale and line families), each with how many materials, types, views, templates and phase graphics use it, and its `value` in model values.

No parameters.

### `fill_pattern_get`

*query* — One project fill pattern: its definition and everything that uses it (materials, types, views, templates, phase graphics).

| Parameter | Type | | Description |
|---|---|---|---|
| `pattern` | id or name | required | Fill pattern id or name. |

### `fill_pattern_create`

*edit* — Create a project fill pattern from line families: a unique name (not a built-in code), drafting (sized on paper, the default) or model (sized in the world, scaling with the view), a scale and the families.

| Parameter | Type | | Description |
|---|---|---|---|
| `lines` | [object, …] | required | The line families (1 to 128). |
| `name` | string | required | Name (unique, not a built-in pattern's code). |
| `description` | string | optional | Description. |
| `kind` | string | optional | `drafting` (default: millimeters on paper) or `model` (millimeters in the world). |
| `scale` | number | optional | Multiplies every length (0.01 to 100; default 1). |

### `fill_pattern_set`

*edit* — Change a project fill pattern: name, description, kind, scale or line families. What is drawn with it follows.

| Parameter | Type | | Description |
|---|---|---|---|
| `pattern` | id or name | required | Fill pattern id or name. |
| `description` | string | optional | New description. |
| `kind` | string | optional | `drafting` or `model`. |
| `lines` | [object, …] | optional | New line families, in place of all of them. |
| `name` | string | optional | New name. |
| `scale` | number | optional | Multiplies every length (0.01 to 100). |

### `fill_pattern_duplicate`

*edit* — Copy a project fill pattern under a new name (default: the name with a number).

| Parameter | Type | | Description |
|---|---|---|---|
| `pattern` | id or name | required | Fill pattern id or name. |
| `name` | string | optional | Name of the copy. |

### `fill_pattern_delete`

*edit* — Delete a project fill pattern. Refused while something uses it, unless `replace_with` names the pattern (a built-in code or another project pattern) that takes its place there, in the same undo step.

| Parameter | Type | | Description |
|---|---|---|---|
| `pattern` | id or name | required | Fill pattern id or name. |
| `replace_with` | string | optional | The pattern that takes its place where it is used. |

### `fill_pattern_import`

*edit* — Import fill patterns from a `.pat` file (the plain-text hatch pattern format CAD programs share: `*NAME, description` headers, then lines of `angle, x-origin, y-origin, delta-x, delta-y [, dashes…]`). All the file's patterns or those named; each pattern is drafting or model as its `;%TYPE=MODEL` marker says unless `kind` is given, lengths in the file's `;%UNITS=` (else millimeters) unless `units` is given. Patterns with a problem are left out and listed in `errors` with their line. With `preview`, nothing changes: the result lists what the file holds.

| Parameter | Type | | Description |
|---|---|---|---|
| `kind` | string | optional | `drafting` or `model` for every pattern, in place of the file's markers. |
| `names` | [string, …] | optional | Names of the patterns to import (default: all). |
| `on_clash` | JSON | optional | A name the project already has: `rename` (default), `replace` or `skip`. |
| `path` | string | optional | Path of the .pat file (or give `text`). |
| `preview` | boolean | optional | Only read the file and list its patterns. |
| `scale` | number | optional | The scale given to every imported pattern (0.01 to 100; default 1). |
| `text` | string | optional | The file's text (in place of `path`). |
| `units` | string | optional | Units of the file's lengths: `auto` (default: its `;%UNITS=` marker, else millimeters), `mm` or `inch`. |

### `fill_pattern_export`

*file* — Write project fill patterns as `.pat` text (millimeters, each pattern's scale applied, a `;%TYPE=` marker per pattern): to `path` when given; the text is returned either way.

| Parameter | Type | | Description |
|---|---|---|---|
| `path` | string | optional | Path of the .pat file to write. |
| `patterns` | [id or name, …] | optional | Fill pattern ids or names (default: all the project's). |

### `profiles_list`

*query* — The project's structural profiles, optionally of one material or shape: id, name, material, shape, area (m²), mass per meter in steel (kg/m) and the number of column and beam types using each.

| Parameter | Type | | Description |
|---|---|---|---|
| `material` | string | optional | Only profiles of this material: `steel`, `cold_formed`, `timber` or `concrete`. |
| `shape` | string | optional | Only profiles of this shape code (`i`, `channel`, `rect`, `sketch`…). |

### `profile_get`

*query* — One structural profile: its material, shape and parameters (and the sketch of a sketched one), the outline (outer loop counter-clockwise, holes clockwise, meters, centered on its bounding box), its section properties (SI units; mass per length with the usual density of its material), its catalog entry and the names of the types using it.

| Parameter | Type | | Description |
|---|---|---|---|
| `profile` | id or name | required | Profile id or name. |

### `profile_create`

*edit* — Create a structural profile: a standard shape of a material (its default dimensions, changed by `params`), or a sketch drawn with `curves` (and `openings`), whose points keep the drawn coordinates and which declares `sketch_params`. Without a name it gets one from its shape and size.

| Parameter | Type | | Description |
|---|---|---|---|
| `material` | string | required | Material: `steel`, `cold_formed`, `timber` or `concrete`. |
| `curves` | [JSON, …] | optional | The closed outline of a sketched profile, in meters. |
| `name` | string | optional | Name (unique, ignoring case; default: from the shape and its size). |
| `openings` | [[JSON, …], …] | optional | The openings of a sketched profile: each a closed loop of curves inside the outline. |
| `params` | object | optional | Parameter values by name, meters (degrees for `angle`), over the shape's defaults; see `profile_get` `params` for the names. |
| `shape` | string | optional | Shape code: `i`, `i_asym`, `channel`, `t`, `angle`, `cold_u`, `cold_lipped_u`, `cold_z`, `cold_hat`, `rect_tube`, `round_tube`, `rect`, `round`, `concrete_t`, `concrete_l`, `annular_sector` (the piece of a ring between `r_in` and `r_out` turning by `sweep` degrees) or `sketch` (the default with `curves`; required without them). |
| `sketch_params` | [object, …] | optional | Numeric parameters a sketched profile declares (lengths). |

### `profile_set`

*edit* — Change a structural profile: its name, material, parameter values or, for a sketched one, its sketch (a merge patch over the `sketch` that `profile_get` shows: `params`, `outline` and `openings`). A change that leaves no valid outline is refused. The column and beam types using it follow; a profile whose size or material changes is no longer its catalog entry.

| Parameter | Type | | Description |
|---|---|---|---|
| `profile` | id or name | required | Profile id or name. |
| `material` | string | optional | New material: `steel`, `cold_formed`, `timber` or `concrete`. |
| `name` | string | optional | New name (unique, ignoring case). |
| `params` | object | optional | Parameter values by name, meters (degrees for `angle`). |
| `sketch` | JSON | optional | Merge patch over a sketched profile's sketch. |

### `profile_duplicate`

*edit* — Copy a structural profile under a new name (default: the name with a number).

| Parameter | Type | | Description |
|---|---|---|---|
| `profile` | id or name | required | Profile id or name. |
| `name` | string | optional | Name of the copy (unique, ignoring case). |

### `profile_delete`

*edit* — Delete a structural profile. Refused while column or beam types use it, unless `replace_with` names the profile they take instead, in the same undo step.

| Parameter | Type | | Description |
|---|---|---|---|
| `profile` | id or name | required | Profile id or name. |
| `replace_with` | id or name | optional | The profile (id or name) the types using it take instead. |

### `profile_catalog`

*query* — The built-in profile catalogs (standard steel, cold-formed, timber and hollow sections). Without `series`: each series' `code`, `name`, `material`, `shape` and row `count`. With it: the series' `code`, `name` and `rows`, each a `designation`, the `name` an imported profile gets and its `params` (meters; degrees for `angle`). Catalogs are not part of the project; `profile_import` copies rows in.

| Parameter | Type | | Description |
|---|---|---|---|
| `search` | string | optional | Only rows whose designation contains this text (ignoring case). |
| `series` | string | optional | Series code (`W`, `HP`, `IPE`, `HEA`, `HEB`, `NBR6355-Ue`, `SHS`…; ignoring case). |

### `profile_import`

*edit* — Copy rows of a built-in catalog series into the project as profiles that remember their catalog entry, in one undo step (none when nothing is imported). A row whose profile name is taken keeps the existing profile (`on_clash: "keep"`, the default) or comes in under a free name (`"rename"`). Returns the `imported` profiles (`id`, `name`), the names `kept` and the designations the series does not have (`unknown`).

| Parameter | Type | | Description |
|---|---|---|---|
| `designations` | [string, …] | required | Designations to import, as `profile_catalog` lists them (ignoring case). |
| `series` | string | required | Series code (see `profile_catalog`). |
| `on_clash` | string | optional | `keep` (default) or `rename`. |

### `profile_type_create`

*edit* — Make a column or beam type of a structural profile: a copy of the category's first type given the profile, its width and height becoming the profile's bounding box. Returns its `id` and `name`.

| Parameter | Type | | Description |
|---|---|---|---|
| `category` | string | required | `column` or `beam`. |
| `profile` | id or name | required | Profile id or name. |
| `material` | string | optional | Material name (default: the first project material of the profile's class, metal, wood or concrete; else the template's concrete). |
| `name` | string | optional | Type name (unique in its category; default: the profile's name, made unique). |

### `structural_type_for`

*edit* — The column or beam type new members of a structural material get: the first type of that material, else a default made in one undo step (steel `W200x26,6`, timber `MLC 9x32` from the profile catalog, concrete a 0.20 x 0.40 rectangle named `20x40`). Returns its `id`, `name` and whether it was `created`.

| Parameter | Type | | Description |
|---|---|---|---|
| `category` | string | required | `column` or `beam`. |
| `material` | string | required | `concrete`, `steel` or `timber`. |

### `steel_building_create`

*edit* — Create a parametric steel building: portal frames (two columns and the rafters of a gable or mono-pitch roof) every `bay_spacing` along it, purlins on the roof and girts on both long sides, as ordinary columns and beams the building keeps in step with its parameters (`element_set` regenerates them, keeping their ids). A member edited or deleted by hand is detached and left alone. Default types come from the profile catalog (`W310x38,7` for columns and rafters, `Ue 150x60x20x2,00` for purlins and girts), made when missing. Out-of-range values are refused. One undo step; returns the building's `id` and its `members`.

| Parameter | Type | | Description |
|---|---|---|---|
| `bay_spacing` | number | optional | Frame to frame, meters, 1 to 30 (default 6). |
| `bays` | integer | optional | Bays, 1 to 100 (default 5); there is one frame more. |
| `column_type` | id or name | optional | Column type id or name (default: catalog `W310x38,7`). |
| `eave_height` | number | optional | Underside of the rafters at the columns above the level, meters, 1 to 50 (default 6). |
| `girts` | object | optional | Girts on both long sides `{ "type", "spacing" }`, or `null` for none (default: on). |
| `level` | id or name | optional | Level id or name (default: the lowest level). |
| `origin` | [x, y] | optional | Plan point `[x, y]` of the first frame's left column (default `[0, 0]`). |
| `purlins` | object | optional | Purlins `{ "type", "spacing" }`, or `null` for none (default: on). |
| `rafter_type` | id or name | optional | Rafter (beam) type id or name (default: catalog `W310x38,7`). |
| `roof` | string | optional | `gable` (ridge at mid-span, default) or `mono` (rising toward the far columns). |
| `rotation` | number | optional | Turn of the span direction, degrees counter-clockwise (default 0). |
| `slope_percent` | number | optional | Roof slope, percent, 0 to 100 (default 10). |
| `span` | number | optional | Column axis to column axis, meters, 2 to 100 (default 20). |

### `wall_framing_create`

*edit* — Frame straight walls in light steel or wood, one framing per wall. Studs stand every `spacing` from the wall start (plus one flush with each end) between a bottom track or plate and one top track (two plates for wood). Each door or window gets king and jack studs, a header, a sill (windows) and cripples. Blocking at mid-height is optional. The studs are columns and the rest are beams, all ordinary elements that the framing keeps in step with its parameters, its wall and the wall's doors and windows (`element_set` on the framing regenerates them, keeping their ids). A member edited or deleted by hand is detached and left alone, and deleting the wall deletes its framing and members. The framing and its members follow the wall's phases, design option and level. Default types come from the profile catalog, made when missing: - light steel: `Ue 90x40x12x0,95` studs, `U 92x38x0,95` tracks and `Ue 140x40x12x0,95` headers; - wood: `Peça 6x8` studs and plates, and `Peça 6x16` headers. The tracks and plates lie flat: their default beam type has the profile turned 90 degrees, so its depth runs across the wall and its width (38 mm, 6 cm) is their height. A given type is used as it draws: the height of its section is the height of the track. Curved walls, walls already framed and out-of-range values are refused before anything is made. One undo step; returns `framings`, each with its `id`, `wall` and `members`.

| Parameter | Type | | Description |
|---|---|---|---|
| `walls` | [id, …] | required | Ids of the straight walls to frame (each gets its own framing). |
| `blocking` | boolean | optional | A row of blocking at mid-height between the studs (default off). |
| `double_top` | boolean | optional | An extra top plate (wood only; default on for wood). |
| `header_type` | id or name | optional | Beam type of the headers and sills, id or name (default: the system's catalog header). |
| `spacing` | number | optional | Stud spacing on center from the wall start, meters, 0.2 to 1.2 (default 0.4 for light steel, 0.6 for wood). |
| `stud_type` | id or name | optional | Column type of the studs, id or name (default: the system's catalog stud). |
| `system` | string | optional | `light_steel` (default) or `wood`. |
| `track_type` | id or name | optional | Beam type of the tracks or plates and of the blocking, id or name (default: the system's catalog track or plate). |

### `paint`

*edit* — Paint a face of an element with a material: the face at `point` (and those touching it in its plane, or the rest of a curved surface). It shows the material's colors, surface pattern and texture over the type's material; painting a painted surface again replaces its paint. The paint follows the face when the element changes (the face turned like it that passes nearest its point).

| Parameter | Type | | Description |
|---|---|---|---|
| `element` | integer | required | Element id. |
| `material` | id or name | required | Material name (or index in `materials_list`). |
| `point` | [number, …] | required | A point on the face, `[x, y, z]` meters (within 5 cm of it). |
| `normal` | [number, …] | optional | The face's outward normal, to pick between faces meeting at the point (a 3D pick gives it); default: the nearest face. |

### `paint_remove`

*edit* — Take the paint off a face of an element (the one at `point`), or every paint of the element when no point is given.

| Parameter | Type | | Description |
|---|---|---|---|
| `element` | integer | required | Element id. |
| `normal` | [number, …] | optional | The face's outward normal (see `paint`). |
| `point` | [number, …] | optional | A point on the painted face, `[x, y, z]` meters (default: every paint of the element). |

### `paints_list`

*query* — The painted faces of an element (or of every element): each paint's material, point and normal, and the faces and area it covers now (none when no face is turned like it any more).

| Parameter | Type | | Description |
|---|---|---|---|
| `element` | integer | optional | Element id (default: every element with paints). |

## Levels and views

### `level_create`

*edit* — Add a level (default: 3 m above the highest, named "Level n") with its floor and ceiling plans. With `above` or `below` it is a new storey next to that level: the levels beyond it move away by the new storey's height, so the other storeys keep theirs and the order of the levels stays.

| Parameter | Type | | Description |
|---|---|---|---|
| `above` | id or name | optional | Add the level above this one; the levels above it move up by `height`. |
| `below` | id or name | optional | Add the level below this one; the levels below it move down by `height`. |
| `elevation` | number | optional | Elevation in meters (not with `above` or `below`). |
| `height` | number | optional | With `above` or `below`: the distance to that level, meters (default: the height of the storey next to it on that side, else 3 m). |
| `name` | string | optional | Level name (default: "Level n"). |
| `plans` | boolean | optional | Also create its floor and ceiling plans (default true). |

### `level_set`

*edit* — Rename a level, change its elevation or the height up to the next level (what is attached to a moved level moves along). A new `height` moves every level above it by the difference, so the levels keep their order and the other storeys their heights. Renaming it leaves its plans' names as they are: a level and its views are named apart.

| Parameter | Type | | Description |
|---|---|---|---|
| `level` | id or name | required | A level, type, view or material: its numeric id or its name. |
| `elevation` | number | optional | New elevation, meters (not with `height`). |
| `height` | number | optional | New height from this level up to the next one, meters; the levels above move along. Refused for the highest level. |
| `name` | string | optional | New name. |

### `level_delete`

*edit* — Delete a level no element is on, with its plan views; walls rising to it keep their current height, unconnected. The last level is kept.

| Parameter | Type | | Description |
|---|---|---|---|
| `level` | id or name | required | A level, type, view or material: its numeric id or its name. |

### `view_create`

*edit* — Create a view. `kind` is `floor_plan` or `ceiling_plan` (with `level`), `section` (with `start`, `end` and optional `depth`), `elevation` (with `direction`: north, south, east or west), `structural_plan` (with `level`: a formwork plan of the structure, tagged), `plumbing_plan`, `electrical_plan`, `communications_plan` and `lightning_plan` (with `level`: the pipes, the conduits and devices, the structured cabling, or the lightning protection, over the architecture, tagged), `three_d`, `analytical` (a 3D view of the analytical model), `rebar` (a 3D view of the reinforcement), `legend` (all types), `drafting` (a 2D view independent of the model, showing only the text, dimensions and imported drawings placed in it; 1:10 by default), `schedule` (with `category`), `panel_diagram` (with `panel`: the single-line diagram and load schedule of a distribution board; default the only board) or `sheet` (A3, ANSI B in imperial projects, numbered after the others, with the first title block type).

| Parameter | Type | | Description |
|---|---|---|---|
| `kind` | string | required | `floor_plan`, `ceiling_plan`, `structural_plan`, `plumbing_plan`, `electrical_plan`, `communications_plan`, `lightning_plan`, `section`, `elevation`, `three_d`, `analytical`, `rebar`, `legend`, `drafting`, `schedule`, `panel_diagram` or `sheet`. |
| `category` | string | optional | Schedules: the category scheduled (default `wall`). |
| `depth` | number | optional | Section depth beyond the cut line, meters (default 8). |
| `direction` | string | optional | Elevations: `north`, `south` (default), `east` or `west`. |
| `end` | [x, y] | optional | Sections: end of the cut line. |
| `level` | id or name | optional | Plans: level id or name (default: the lowest). |
| `name` | string | optional | View name (default: by kind). |
| `panel` | integer | optional | Distribution board element id, for a panel diagram. |
| `scale` | integer or string | optional | Scale: the denominator (e.g. 50 for 1:50), or text such as `"1:50"` or an imperial label like `"1/4\" = 1'-0\""` (48). |
| `start` | [x, y] | optional | Sections: start of the cut line. |

### `view_set`

*edit* — Change a view: name, scale, phase and phase filter, and its `params` as `view_get` shows them (merged: cut height, section depth, legend types, schedule fields, sort, filter and totals, sheet number and size, viewport positions…), the crop, the sun, the render settings and the shading.

| Parameter | Type | | Description |
|---|---|---|---|
| `view` | id or name | required | A level, type, view or material: its numeric id or its name. |
| `crop` | [[x, y], [x, y]] | optional | Crop rectangle: two opposite corners in the view's drawing coordinates (plans, sections and elevations draw nothing of the model outside it), or `null` to show everything. For a detail view this is its callout in the parent; for a drafting view a callout points to, it moves that callout. |
| `detail` | string | optional | Detail level: `coarse` (layered elements cut as one plain region; component pieces set for it), `medium` or `fine`. |
| `name` | string | optional | New name. A plan's name is its own: its level and the level's other plans keep theirs. |
| `params` | JSON | optional | Fields of the view's parameters to change, shaped like `params` in `view_get`. |
| `phase` | id or name | optional | Phase the view shows (id or name; plans, elevations, sections, 3D views, schedules). |
| `phase_filter` | id or name | optional | Phase filter (id or name), or `null` for none (every status by category). |
| `render` | JSON | optional | The view's photorealistic render settings (3D views only): the fields to change, shaped like `render` in `view_get` (`samples` 1 to 16384, `max_depth` 1 to 64, `width`/`height` 16 to 8192 pixels and at most 7680x4320 in all, `fov_deg` 10 to 120, `exposure` -10 to 10 stops, `tone_map` `Filmic` or `Standard` (or lowercase), `clamp` 0 (off) to 1000, `clay`, and `environment` with `sky_strength` and `sun_strength` 0 to 100, `ground_color` [r, g, b] and `ground_plane`); a view without settings starts from the defaults. `null` removes them. |
| `scale` | integer or string | optional | Scale: the denominator (e.g. 100 for 1:100), or text such as `"1:100"` or an imperial label like `"1/8\" = 1'-0\""` (96). |
| `shading` | JSON | optional | How the view shades faces and casts shadows in the shaded, consistent colors and textures styles (plans, ceiling plans, 3D views, elevations and sections; not photorealistic renders): the fields to change, shaped like `shading` in `view_get`: `ambient`, the light of a face turned away from the light, 0.3 (dark grey) to 1 (no shading; default 0.6), and `shadow`, how dark cast shadows are, 0 (none) to 1 (black; default 0.55, a shaded shadow's grey being `1 - shadow`). |
| `sun` | object | optional | The view's sun (plans, ceiling plans, 3D views, elevations and sections): the fields to change, shaped like `sun` in `view_get` (a view without a sun starts from a still sun on 2026-06-21 at 10:00), `{"preset": name}` to take a preset, or `null` to remove it. With `shadows`, the sun casts shadows in 3D views and, in the shaded, consistent colors and textures styles, in plans, elevations and sections (a floor plan from the whole model, a section from what lies beyond its line, neither onto what is indoors; ceiling plans draw none). |
| `visual_style` | string | optional | Visual style: `wireframe` (every edge, no faces), `hidden_line` (white faces), `shaded` (white faces greyed by the light, with the sun's shadows), `consistent_colors` (material colors, unlit), `textures` (appearance colors and surface patterns), or `default` (plans hidden line, other views shaded). |

### `schedule_get`

*query* — A schedule's definition: the `fields` it shows in order, its `filters` (`field`, `op`, `value`), its `sorting` levels (`field`, `descending`, `header`, `footer`, `blank_line`), whether every element has its own row (`itemize`), whether the totals row shows (`totals`), each column's format (`columns`: `heading`, `align`, `hidden`, `total`) and the table's `appearance` on paper; and the fields `available` to schedules of its category. `schedule_set` changes any of them.

| Parameter | Type | | Description |
|---|---|---|---|
| `view` | id or name | required | Schedule view id or name. |

### `schedule_set`

*edit* — Change a schedule's definition; every part is optional and replaces the part it names, the others staying as they are. `fields` are the columns in order. `filters` keep the rows that pass every one: each compares a `field` with a `value` by `op` (`equals`, `not_equals`, `contains`, `not_contains`, `begins_with`, `not_begins_with`, `greater`, `greater_or_equal`, `less`, `less_or_equal`, `has_value`, `has_no_value`); numbers compare as numbers in SI units (lengths in meters), texts without case. `sorting` orders the rows by up to four levels, the first outermost; a level with `header`, `footer` or `blank_line` groups the rows alike in it under a heading, over a footer (the group's count and totals) or apart by a blank line. With `itemize` false, the elements alike in every sorting level share one row (with no sorting, those alike in every column shown but `Count` and the totaled ones): values that differ among them read as varying and totaled columns add up (the `Count` field says how many). `columns` formats the columns of the fields named (`heading`, `align`: `Auto`, `Left`, `Center`, `Right`; `hidden`: a hidden field still sorts and filters; `total`: add the column up in footers and totals), `totals` shows or hides the totals row, and `appearance` is how the table is drawn on sheets and exports (`grid_lines`, `outline`, `show_title`, `show_headers`, and the `title_size`, `header_size` and `text_size` in millimeters). One undo step; returns the definition as `schedule_get` does.

| Parameter | Type | | Description |
|---|---|---|---|
| `view` | id or name | required | Schedule view id or name. |
| `appearance` | JSON | optional | How the table looks on paper: a patch over `{grid_lines, outline, show_title, show_headers, title_size, header_size, text_size}`. |
| `columns` | JSON | optional | Column formats, `[{"field": "Area", "heading": "Net area", "align": "Right", "total": true}, …]`: the fields named take the format given, the others keep theirs (`[]` clears every format). |
| `fields` | [string, …] | optional | The fields shown, in order (names as `schedule_get` lists them in `available`). |
| `filters` | JSON | optional | The filters, `[{"field": "Level", "op": "equals", "value": "Térreo"}, …]` (at most 8; `[]` for none). |
| `itemize` | boolean | optional | One row per element (default true); false gathers the elements alike in every sorting level in one row (with no sorting, those alike in every column shown but the count and the totaled ones). |
| `sorting` | JSON | optional | The sorting levels, `[{"field": "Type", "header": true, "footer": true}, …]` (at most 4; `[]` for none). |
| `totals` | boolean | optional | Show the totals row. |

### `view_duplicate`

*edit* — Copy a view (a copied sheet starts without viewports), in one undo step. With `detailing`, what is drawn in it comes along: detail lines, filled and masking regions, text notes, dimensions (still measuring the same elements), level marks, detail components and imported drawings; the result lists them in `copied`.

| Parameter | Type | | Description |
|---|---|---|---|
| `view` | id or name | required | The view to copy (id or name). |
| `detailing` | boolean | optional | Copy the elements drawn in the view too (default: only the view). |

### `view_delete`

*edit* — Delete a view (and its viewports on sheets; dimensions drawn in it go too).

| Parameter | Type | | Description |
|---|---|---|---|
| `view` | id or name | required | A level, type, view or material: its numeric id or its name. |

### `callout_create`

*edit* — Draw a callout in a plan, ceiling plan, section or elevation: a rectangle (two opposite corners in the view's drawing coordinates) standing for a detail view of that region. By default a new detail view is made: of the parent's kind (it stays live with the model, following the parent's level, section line or direction), cropped to the rectangle, at a larger scale, with its own detail lines, regions, detail components, text and dimensions. With `drafting`, the callout points to that existing drafting view instead (no crop). The parent draws the rectangle with rounded corners and a tag with the detail number and the sheet number once the view is placed on a sheet. Moving the callout is changing the detail view's `crop` (`view_set`); deleting the detail view removes it (`callout_remove` keeps the view).

| Parameter | Type | | Description |
|---|---|---|---|
| `parent` | id or name | required | The view the callout is drawn in (id or name): a floor or ceiling plan, a section or an elevation. |
| `rect` | [[x, y], [x, y]] | required | Two opposite corners of the rectangle, in the parent's drawing coordinates (meters; in sections and elevations `y` is the height). |
| `drafting` | id or name | optional | An existing drafting view (id or name) the callout points to instead of a new detail view. |
| `name` | string | optional | Name of the detail view (default "Detail N"). |
| `scale` | integer or string | optional | Scale of the detail view: the denominator, or text such as `"1:10"` or an imperial label like `"1 1/2\" = 1'-0\""` (default: a standard scale of the project's units about five times larger than the parent's, 1:10 under 1:50). |

### `callout_remove`

*edit* — Remove a view's callout from its parent; the view stays (a detail view keeps its crop and becomes an ordinary view of its kind).

| Parameter | Type | | Description |
|---|---|---|---|
| `view` | id or name | required | The detail or drafting view the callout stands for (id or name). |

### `callouts_list`

*query* — The callouts drawn in a view: the detail or drafting view each stands for and its rectangle.

| Parameter | Type | | Description |
|---|---|---|---|
| `view` | id or name | required | The parent view (id or name). |

### `sun_position`

*query* — Where the sun stands: for a view with a sun, at its current frame (or its lighting direction); else on `date` at `time` (local) at the project's site, or at `latitude`, `longitude` and `utc_offset` given here (a longitude given without an offset takes its nominal time zone). Returns the azimuth (degrees clockwise from true north), the altitude (degrees above the horizon, no refraction), `above_horizon`, the unit `direction` towards the sun in project coordinates (true north applied), sunrise and sunset that day (`HH:MM`, or null with `polar` `day` or `night`), and `site_known` (false: the equator at Greenwich).

| Parameter | Type | | Description |
|---|---|---|---|
| `date` | string | optional | The date, `YYYY-MM-DD`. |
| `latitude` | number | optional | Latitude, degrees (-90 to 90, north positive), instead of the site's. |
| `longitude` | number | optional | Longitude, degrees (-180 to 180, east positive), instead of the site's. |
| `time` | string | optional | The local time of day, `HH:MM`. |
| `utc_offset` | number | optional | Local time's offset from UTC, hours (-12 to 14), instead of the site's. |
| `view` | id or name | optional | A view with a sun (id or name); or give `date` and `time`. |

### `sun_frames`

*query* — The frames of a view's sun study, in time order: `count` (all of them, at most 5000; 0 for lighting, 1 for a still sun) and the first `limit` as `{index, date, time, azimuth, altitude}`.

| Parameter | Type | | Description |
|---|---|---|---|
| `view` | id or name | required | A view with a sun (id or name). |
| `limit` | integer | optional | Most frames listed (default 500, at most 5000). |

### `sun_presets_list`

*query* — The project's sun presets: each `name` with its mode in the form `view_set` `sun` takes (a view takes one with `"sun": {"preset": name}`).

No parameters.

### `sun_preset_save`

*edit* — Save a sun preset, in one undo step: the mode and its values (dates, times, step or light direction; not the frame, path, shadows or ground) of a view's sun, or of `sun` applied to the default sun. A preset of the same name is replaced only with `replace`; a project keeps at most 200.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name of the preset. |
| `replace` | boolean | optional | Replace a preset of the same name (default false: refused). |
| `sun` | object | optional | The sun to save, as `view_set` takes it. |
| `view` | id or name | optional | A view whose sun is saved (id or name); or give `sun`. |

### `sun_preset_rename`

*edit* — Rename a sun preset, in one undo step.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | The preset's name. |
| `new_name` | string | required | Its new name (no other preset's). |

### `sun_preset_delete`

*edit* — Delete a sun preset, in one undo step (views that took it keep their sun).

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | The preset's name. |

### `sheet_place_view`

*edit* — Place a view on a sheet at `center` (paper millimeters; default left of center). A view can be on only one sheet; sheets cannot be placed.

| Parameter | Type | | Description |
|---|---|---|---|
| `sheet` | id or name | required | A level, type, view or material: its numeric id or its name. |
| `view` | id or name | required | A level, type, view or material: its numeric id or its name. |
| `center` | [x, y] | optional | Center of the viewport on the sheet, paper millimeters. |

### `sheet_remove_viewport`

*edit* — Remove a view from a sheet.

| Parameter | Type | | Description |
|---|---|---|---|
| `sheet` | id or name | required | A level, type, view or material: its numeric id or its name. |
| `view` | id or name | required | A level, type, view or material: its numeric id or its name. |

### `sheet_set`

*edit* — Change a sheet: its number, paper (`size`, `portrait`), title block, per-sheet field values and issue date.

| Parameter | Type | | Description |
|---|---|---|---|
| `sheet` | id or name | required | The sheet: id or name. |
| `fields` | object | optional | Values of the title block's per-sheet fields by parameter name: a text sets one, `null` clears it. |
| `issue_date` | string | optional | Issue date, `YYYY-MM-DD`; empty to clear. |
| `number` | string | optional | Sheet number. |
| `portrait` | boolean | optional | The long side runs up. |
| `size` | string or object | optional | Paper: `"A0"`…`"A4"`, `"ANSI A"`…`"ANSI E"`, `"ARCH A"`…`"ARCH E1"`, `"ARCH E"`, or `{width, height}` in millimeters (50 to 5000). |
| `title_block` | id or name | optional | The title block type (id or name), or `null` for none. |

### `sheet_fields`

*query* — A sheet's title block fields: every field key (`fields`, as `{key}` in title block texts reads it) with its value on this sheet, the title block's per-sheet parameters (`params`: name, label, the sheet's value and the type's default), and the title block's pieces that do not resolve on this paper (`errors`).

| Parameter | Type | | Description |
|---|---|---|---|
| `sheet` | id or name | required | The sheet: id or name. |

### `title_block_create`

*edit* — Create a title block component: a copy of the standard one, or empty with `empty`. Sheets use it through `sheet_set` `title_block`.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name of the component and of its first type. |
| `empty` | boolean | optional | Start with no pieces. |

### `component_sheet_piece_add`

*edit* — Add 2D pieces to a title block or detail component (every type of it), in one step.

| Parameter | Type | | Description |
|---|---|---|---|
| `type` | id or name | required | A type of the title block or detail component: id or name. |
| `name` | string | optional | Name of the first piece added (default `Piece1`, `Piece2`…). |
| `piece` | JSON | optional | The piece, by `kind`: `stroke` (`points` as [{u, v, radius}], `closed`, `weight`, `color`, `fill`, `pattern`), `text` (`x`, `y`, `text` with `{fields}`, `size`, `anchor`: left, center, right or middle, `rotation`), `image` (`x`, `y`, `width`, `height`, and `image_path` or base64 `image`) or, in a detail component, `nested` (`component`, `x`, `y`, `rotation`, `scale`, `mirrored`). A title block's are millimeters from the paper's bottom-left corner, as numbers or expressions over the parameters, `SheetWidth` and `SheetHeight`; a detail component's are meters in its frame. |
| `pieces` | [JSON, …] | optional | More pieces (each as `piece`), added after it in the same step: all of them or, when one is refused, none. |

### `component_sheet_piece_set`

*edit* — Change a 2D piece of a title block or detail component: `piece` is a merge patch over its form in `component_get` `def.sheet` (e.g. `{"Text": {"size": "4"}}`); an image piece's picture is replaced with `image_path` or `image`, not by the patch.

| Parameter | Type | | Description |
|---|---|---|---|
| `index` | integer | required | The piece's index (in `component_get` `def.sheet`). |
| `type` | id or name | required | A type of the title block: id or name. |
| `add` | [JSON, …] | optional | Pieces added in the same step (each as `component_sheet_piece_add` `piece`; e.g. the other chains of a redrawn sketch); the result lists their indices in `added`. |
| `image` | string | optional | A new picture for an image piece: its bytes, base64. |
| `image_path` | string | optional | A new picture for an image piece: its file (PNG, JPEG, GIF, BMP or WebP). |
| `name` | string | optional | New name. |
| `piece` | JSON | optional | Merge patch over the piece's shape (without a picture). |

### `component_sheet_piece_delete`

*edit* — Delete 2D pieces of a title block or detail component, in one step.

| Parameter | Type | | Description |
|---|---|---|---|
| `type` | id or name | required | A type of the title block: id or name. |
| `index` | integer | optional | The piece's index (in `component_get` `def.sheet`). |
| `indices` | [id, …] | optional | More pieces' indices, deleted with `index` in the same step: all of them or, when one does not exist, none. |

## View graphics

### `view_graphics_get`

*query* — The visibility/graphics of a view (or of a view template): every category's visibility and overrides, the applied filters in order, the element overrides and hidden elements, the view's template and what it controls, and what is temporarily hidden or isolated. Overrides are objects like `{"cut_lines": {"color": [255, 0, 0], "weight": 0.5, "pattern": "Dash"}, "projection_lines": {..}, "cut_fill": {"color": [..], "pattern": "Solid"}, "surface_fill": {..}, "halftone": true, "transparency": 0.5}` (absent attributes are not overridden).

| Parameter | Type | | Description |
|---|---|---|---|
| `template` | id or name | optional | Or a view template id or name. |
| `view` | id or name | optional | View id or name. |

### `view_graphics_set`

*edit* — Show or hide a category in a view (or view template) and change its graphic overrides: `overrides` is merged into the current ones (`null` resets one attribute; see `view_graphics_get`), `reset` clears them first. Refused when the view's template controls its categories.

| Parameter | Type | | Description |
|---|---|---|---|
| `category` | string | required | Category (`wall`, `door`, `room`, `dimension`…). |
| `overrides` | JSON | optional | Overrides to merge. |
| `reset` | boolean | optional | Clear the category's overrides before merging. |
| `template` | id or name | optional | Or a view template id or name. |
| `view` | id or name | optional | View id or name. |
| `visible` | boolean | optional | Show (`true`) or hide (`false`) the category. |

### `element_override_set`

*edit* — Override the graphics of elements in one view (they keep their category's and filters' graphics for what is not overridden): `overrides` is merged into each element's (`null` resets an attribute), `reset` clears them first, so `reset` alone removes the overrides.

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |
| `view` | id or name | required | View id or name. |
| `overrides` | JSON | optional | Overrides to merge (see `view_graphics_get`). |
| `reset` | boolean | optional | Clear the elements' overrides before merging. |

### `element_hide_in_view`

*edit* — Hide elements in one view (`hidden: false` shows them again). Hidden elements stay in the model and in other views.

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |
| `view` | id or name | required | View id or name. |
| `hidden` | boolean | optional | Hide (default) or unhide. |

### `filters_list`

*query* — The project's view filters with their categories and rules.

No parameters.

### `filter_create`

*edit* — Create a view filter: the elements of `categories` (all when empty) that pass its rules — all of them, or any with `any`; a filter without rules matches every element of its categories. Apply it to views with `view_filter_add`.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name (unique). |
| `any` | boolean | optional | Match when any rule passes (default: all must pass). |
| `categories` | [string, …] | optional | Categories (`wall`, `door`…); empty for all. |
| `rules` | [object, …] | optional | Rules on parameters. |

### `filter_set`

*edit* — Change a view filter: name, categories, rules (replaced as a whole) or how rules combine. Views that apply it follow.

| Parameter | Type | | Description |
|---|---|---|---|
| `filter` | id or name | required | Filter id or name. |
| `any` | boolean | optional | Match when any rule passes (`false`: all must pass). |
| `categories` | [string, …] | optional | New categories (empty for all). |
| `name` | string | optional | New name. |
| `rules` | [object, …] | optional | New rules (replace the old ones). |

### `filter_delete`

*edit* — Delete a view filter (views and templates stop applying it).

| Parameter | Type | | Description |
|---|---|---|---|
| `filter` | id or name | required | Filter id or name. |

### `filter_elements`

*query* — The elements a view filter matches (only those drawn in `view` when given).

| Parameter | Type | | Description |
|---|---|---|---|
| `filter` | id or name | required | Filter id or name. |
| `view` | id or name | optional | Only elements drawn in this view. |

### `view_filter_add`

*edit* — Apply a filter to a view (or view template), at `index` in the list (default last: the first matching filter wins for each attribute), hiding what it matches (`visible: false`) or overriding its graphics.

| Parameter | Type | | Description |
|---|---|---|---|
| `filter` | id or name | required | Filter id or name. |
| `index` | integer | optional | Position in the view's filter list (0 = first). |
| `overrides` | JSON | optional | Overrides of the matching elements (see `view_graphics_get`). |
| `template` | id or name | optional | Or a view template id or name. |
| `view` | id or name | optional | View id or name. |
| `visible` | boolean | optional | Show (default) or hide the matching elements. |

### `view_filter_set`

*edit* — Change how a view (or view template) applies a filter: visibility of the matching elements, whether it is enabled, and its overrides (merged; `reset` clears them first).

| Parameter | Type | | Description |
|---|---|---|---|
| `filter` | id or name | required | Filter id or name. |
| `enabled` | boolean | optional | Apply the filter (`false` keeps it listed without effect). |
| `overrides` | JSON | optional | Overrides to merge (see `view_graphics_get`). |
| `reset` | boolean | optional | Clear the overrides before merging. |
| `template` | id or name | optional | Or a view template id or name. |
| `view` | id or name | optional | View id or name. |
| `visible` | boolean | optional | Show (`true`) or hide (`false`) the matching elements. |

### `view_filter_remove`

*edit* — Stop applying a filter in a view (or view template).

| Parameter | Type | | Description |
|---|---|---|---|
| `filter` | id or name | required | Filter id or name. |
| `template` | id or name | optional | Or a view template id or name. |
| `view` | id or name | optional | View id or name. |

### `view_filter_move`

*edit* — Move a filter to `index` in a view's (or view template's) filter list (0 = first, the highest priority).

| Parameter | Type | | Description |
|---|---|---|---|
| `filter` | id or name | required | Filter id or name. |
| `index` | integer | required | New position (0 = first). |
| `template` | id or name | optional | Or a view template id or name. |
| `view` | id or name | optional | View id or name. |

### `templates_list`

*query* — The project's view templates: the properties each holds, which of them it controls (`include`) and the views that follow it.

No parameters.

### `template_create`

*edit* — Create a view template, from the properties of `view` when given (scale, category graphics, filters, plan cut height, building services display, structural display), else with defaults. Every property is controlled (`include`) until changed with `template_set`.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name (unique). |
| `view` | id or name | optional | View to take the properties from. |

### `template_set`

*edit* — Change a view template: name, properties and which of them it controls (`include`, merged: `{"scale": false}` lets views keep their own scale). Views that follow the template change with it, in the same undo step.

| Parameter | Type | | Description |
|---|---|---|---|
| `template` | id or name | required | Template id or name. |
| `cut_height` | number | optional | Plans: cut plane height, meters. |
| `design_options` | [object, …] | optional | The option shown of each design option set (replacing the template's choices; sets left out show automatically). |
| `detail` | string | optional | Detail level: `coarse`, `medium` or `fine`. |
| `discipline` | string | optional | Floor plans: `none`, `plumbing`, `electrical`, `communications` or `lightning` (services over a faint architecture). |
| `include` | JSON | optional | Which properties the template controls: `scale`, `categories`, `filters`, `cut_height`, `discipline`, `structural`, `phase_filter`, `detail`, `visual_style`, `underlay`, `design_options` (true/false). `design_options` is off unless set. |
| `name` | string | optional | New name. |
| `phase_filter` | id or name | optional | Phase filter (id or name), or `null` for none. |
| `scale` | integer or string | optional | Scale: the denominator (e.g. 50 for 1:50), or text such as `"1:50"` or an imperial label like `"1/4\" = 1'-0\""` (48). |
| `structural` | boolean | optional | Floor plans: structural (formwork) display. |
| `structure_tags` | boolean | optional | Floor plans: tag beams and columns. |
| `underlay` | JSON | optional | Plans: other levels' plans drawn under them, merged like `view_set`'s `underlay` (e.g. `{"below": {"level": "Next", "looking": "Down"}}`). |
| `visual_style` | string | optional | Visual style: `wireframe`, `hidden_line`, `shaded`, `consistent_colors`, `textures`, or `default` (each view's usual style). |

### `template_delete`

*edit* — Delete a view template; views that follow it keep their current properties and become free.

| Parameter | Type | | Description |
|---|---|---|---|
| `template` | id or name | required | Template id or name. |

### `view_template_apply`

*edit* — Copy a template's properties into views once (they stay free to change). Views that follow another template are refused.

| Parameter | Type | | Description |
|---|---|---|---|
| `template` | id or name | required | Template id or name. |
| `views` | [id or name, …] | required | View ids or names. |

### `view_template_assign`

*edit* — Make views follow a template (`template: null` frees them): the properties it controls are copied now and whenever the template changes, and cannot be changed in the view.

| Parameter | Type | | Description |
|---|---|---|---|
| `views` | [id or name, …] | required | View ids or names. |
| `template` | id or name | optional | Template id or name, or null to unassign. |

### `view_temporary`

*query* — Temporarily hide or isolate elements or categories in a view, or reset. This is display state, not a model change: it is not saved, not undone and not printed (sheets and exports show the view as it is); the application marks the view while it is active. `action`: `hide` and `isolate` (the elements `ids`), `hide_category` and `isolate_category` (`categories`, and the categories of `ids`), or `reset`. Hiding adds to what is already hidden; isolating replaces it.

| Parameter | Type | | Description |
|---|---|---|---|
| `action` | string | required | `hide`, `isolate`, `hide_category`, `isolate_category` or `reset`. |
| `view` | id or name | required | View id or name. |
| `categories` | [string, …] | optional | Category names. |
| `ids` | [id, …] | optional | Element ids. |

### `view_temporary_apply`

*edit* — Make a view's temporary hiding permanent: hidden elements become hidden in the view and hidden categories are turned off; when isolating, every other element drawn in the view is hidden in it. The temporary state is then reset. One undo step.

| Parameter | Type | | Description |
|---|---|---|---|
| `view` | id or name | required | View id or name. |

## Phases

### `phases_list`

*query* — The project's construction phases in time order, with how many elements each builds and demolishes and how many views show it.

No parameters.

### `phase_create`

*edit* — Add a construction phase, at the end (the latest) or before another. Elements and views keep their phases.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name (unique). |
| `before` | id or name | optional | Insert before this phase (id or name; default: after the last). |
| `description` | string | optional | Description. |

### `phase_set`

*edit* — Rename a phase, change its description, or move it to another place in time (`index`, 0 = earliest). A move that would put an element's demolition before its construction is refused.

| Parameter | Type | | Description |
|---|---|---|---|
| `phase` | id or name | required | Phase id or name. |
| `description` | string | optional | New description. |
| `index` | integer | optional | New position in time order (0 = earliest). |
| `name` | string | optional | New name. |

### `phase_delete`

*edit* — Delete a phase. A phase that elements are built or demolished in, or that views show, needs `merge_into`: they move to that phase (an element then built and demolished in the same phase is temporary). The last remaining phase cannot be deleted.

| Parameter | Type | | Description |
|---|---|---|---|
| `phase` | id or name | required | Phase id or name. |
| `merge_into` | id or name | optional | Phase that takes over its elements and views. |

### `elements_phase_set`

*edit* — Set the phase elements are built in and/or demolished in (`null`: not demolished). A demolition cannot come before the construction. Doors, windows and rebar follow their hosts: they are never built before them and are demolished with them (unless demolished earlier).

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |
| `created` | id or name | optional | Phase built in (id or name). |
| `demolished` | id or name | optional | Phase demolished in (id or name), or `null` to keep it standing. |

### `elements_demolish`

*edit* — Demolish elements in a phase: `phase` when given, else the phase of `view`, else the phase new elements are built in (the last by default). Their doors, windows and rebar go with them (unless demolished earlier). Elements not built by then, or already demolished earlier, are refused.

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |
| `phase` | id or name | optional | Phase id or name. |
| `view` | id or name | optional | Or the view whose phase to use. |

### `phase_creation_set`

*query* — Set the phase new elements are built in by this session (the active view's phase in the application): display state, not saved, not undone. `phase` (id or name), or the phase of `view`; neither resets it to the last phase.

| Parameter | Type | | Description |
|---|---|---|---|
| `phase` | id or name | optional | Phase id or name. |
| `view` | id or name | optional | Or the view whose phase to use. |

### `phase_filters_list`

*query* — The phase filters: for each status (`existing`, `demolished`, `new`, `temporary`) whether views using the filter show it `by_category`, `overridden` (with the status graphics, see `phase_graphics_get`) or `not_displayed`.

No parameters.

### `phase_filter_create`

*edit* — Create a phase filter; statuses not given show `by_category`.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name (unique). |
| `demolished` | string | optional | Demolished elements. |
| `existing` | string | optional | Existing elements: `by_category`, `overridden` or `not_displayed`. |
| `new` | string | optional | New elements. |
| `temporary` | string | optional | Temporary elements. |

### `phase_filter_set`

*edit* — Change a phase filter: its name or how it shows each status. Views using it follow.

| Parameter | Type | | Description |
|---|---|---|---|
| `filter` | id or name | required | Phase filter id or name. |
| `demolished` | string | optional | Demolished elements. |
| `existing` | string | optional | Existing elements: `by_category`, `overridden` or `not_displayed`. |
| `name` | string | optional | New name. |
| `new` | string | optional | New elements. |
| `temporary` | string | optional | Temporary elements. |

### `phase_filter_delete`

*edit* — Delete a phase filter; views and view templates using it are left without one (showing every status by category).

| Parameter | Type | | Description |
|---|---|---|---|
| `filter` | id or name | required | Phase filter id or name. |

### `phase_graphics_get`

*query* — The graphics of each phase status (`existing`, `demolished`, `new`, `temporary`), used by phase filters that show a status `overridden`. Overrides are shaped as in `view_graphics_get`.

No parameters.

### `phase_graphics_set`

*edit* — Change the graphics of a phase status: `overrides` is merged into the current ones (`null` resets an attribute; see `view_graphics_get`), `reset` clears them first.

| Parameter | Type | | Description |
|---|---|---|---|
| `status` | string | required | `existing`, `demolished`, `new` or `temporary`. |
| `overrides` | JSON | optional | Overrides to merge. |
| `reset` | boolean | optional | Clear the status overrides before merging. |

## Design options

### `options_list`

*query* — The project's design option sets with their options (the primary one marked, how many elements each holds) and the option being edited.

No parameters.

### `option_set_create`

*edit* — Add a design option set: a named group of alternatives, of which views show one at a time. It starts with the options named in `options` (the first is primary), or one option.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | Name (unique, without ':'). |
| `options` | [string, …] | optional | Names of its options (default: one, "Option 1"). |

### `option_set_rename`

*edit* — Rename a design option set.

| Parameter | Type | | Description |
|---|---|---|---|
| `name` | string | required | New name (unique, without ':'). |
| `set` | id or name | required | Set id or name. |

### `option_set_delete`

*edit* — Delete a design option set with all its options and the elements in them (the main model stays).

| Parameter | Type | | Description |
|---|---|---|---|
| `set` | id or name | required | Set id or name. |

### `option_create`

*edit* — Add an option to a design option set (at the end).

| Parameter | Type | | Description |
|---|---|---|---|
| `set` | id or name | required | Set id or name. |
| `name` | string | optional | Name, unique in the set (default: "Option n"). |
| `primary` | boolean | optional | Make it the set's primary option. |

### `option_set`

*edit* — Rename a design option, make it its set's primary option, or move it to another place in the set's list (`index`, 0 = first).

| Parameter | Type | | Description |
|---|---|---|---|
| `option` | id or name | required | Option id, name, or "Set: Option". |
| `index` | integer | optional | New position in the set's list. |
| `name` | string | optional | New name (unique in the set, without ':'). |
| `primary` | boolean | optional | `true` makes it the primary option (views showing the set automatically, and exports, then use it). |

### `option_delete`

*edit* — Delete a design option and the elements in it (with what they host). The primary option cannot be deleted: make another option primary first, or delete the whole set.

| Parameter | Type | | Description |
|---|---|---|---|
| `option` | id or name | required | Option id, name, or "Set: Option". |

### `option_accept_primary`

*edit* — Accept a set's primary option: its elements join the main model, and the set is deleted with its other options and their elements.

| Parameter | Type | | Description |
|---|---|---|---|
| `set` | id or name | required | Set id or name. |

### `elements_option_set`

*edit* — Move elements into a design option, or back to the main model with `null`. Doors, windows and other hosted elements follow their host's option (moving a wall moves them); annotations belong to no option.

| Parameter | Type | | Description |
|---|---|---|---|
| `ids` | [id, …] | required | Element ids. |
| `option` | id or name | optional | Option id, name or "Set: Option"; `null` for the main model. |

### `view_options_set`

*edit* — Choose the option a view (plan, elevation, section, 3D view or schedule) shows of a design option set; `null` shows it automatically: the option being edited when it is in that set, else the primary one.

| Parameter | Type | | Description |
|---|---|---|---|
| `set` | id or name | required | Set id or name. |
| `view` | id or name | required | View id or name. |
| `option` | id or name | optional | Option id or name (in that set), or `null` for automatic. |

### `option_active_set`

*query* — Set the design option this session edits (`null`: the main model): new model elements go into it, and views without a choice of their own show it for its set. Display state: not saved, not undone.

| Parameter | Type | | Description |
|---|---|---|---|
| `option` | id or name | optional | Option id, name or "Set: Option"; `null` for the main model. |

## Visual programming

### `graph_run`

*edit* — Run a visual programming graph (a `.ccgraph` file, or the graph as JSON) as one undo step. Elements its nodes create are bound to the node and the item that made them: running the graph again changes them in place (keeping their ids), makes the new ones and deletes the ones it no longer makes. Returns each node's outputs or error, the values of its `output` nodes and how many elements were created, changed and deleted.

| Parameter | Type | | Description |
|---|---|---|---|
| `allow_files` | boolean | optional | Let the graph's commands read and write files (open, save, import, export). Off by default; a graph file cannot turn it on, and it stays off when another command (a plugin's) runs the graph. |
| `brief` | boolean | optional | Leave each node's outputs out of the result (only errors, outputs and counts). |
| `graph` | JSON | optional | The graph itself (the contents of a `.ccgraph` file), instead of `path`. |
| `inputs` | object | optional | Values of the graph's input nodes, by their names. |
| `path` | string | optional | Path of the `.ccgraph` file. |

### `graph_check`

*query* — Check a graph without running it: nodes that cannot run here (unknown commands or nodes, file commands the graph does not allow), wires to ports that do not exist or whose types do not fit, required inputs with no value, and what reading the file fixed.

| Parameter | Type | | Description |
|---|---|---|---|
| `allow_files` | boolean | optional | Check it as `graph_run` with `allow_files` would run it (file commands are then no problem). |
| `graph` | JSON | optional | The graph itself, instead of `path`. |
| `path` | string | optional | Path of the `.ccgraph` file. |

### `graph_nodes`

*query* — The nodes a graph can use: built-in nodes (inputs, math, logic, lists, geometry, element parameters, outputs), the Lua node and a node per engine command, each with its inputs and outputs (name, type, whether required, default). Filter by words of the name or description, or by group.

| Parameter | Type | | Description |
|---|---|---|---|
| `group` | string | optional | Only this group (`Math`, `Lists`, `Creating elements`…). |
| `search` | string | optional | Words that must all appear in the node's name, group or description. |

### `graph_elements`

*query* — The elements a graph made, with the node and item (lacing path) that made each.

| Parameter | Type | | Description |
|---|---|---|---|
| `graph` | string | required | The graph's uid (its file's `uid`, also in `graph_run`'s result). |

### `graph_release`

*edit* — Release elements from the graph that made them: they become ordinary elements, and the next run of the graph makes new ones in their place. Without `elements`, every element of the graph is released.

| Parameter | Type | | Description |
|---|---|---|---|
| `graph` | string | required | The graph's uid. |
| `elements` | [id, …] | optional | Ids of the elements to release (default: all of the graph's). |
