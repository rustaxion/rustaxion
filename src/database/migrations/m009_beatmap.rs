use extension::postgres::Type;
use sea_orm::{ActiveModelTrait, EnumIter, Iterable, Set};
use sea_orm_migration::{prelude::*, schema::*};

use crate::database::{
    self,
    json::{BPMRange, BeatmapDifficulty},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("beatmap_kind"))
                    .values(BeatmapKind::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Beatmap::Table)
                    .if_not_exists()
                    .col(pk_auto(Beatmap::Id))
                    .col(string(Beatmap::Name))
                    .col(integer(Beatmap::Quality))
                    .col(string(Beatmap::Composer))
                    .col(enumeration(
                        Beatmap::Kind,
                        Alias::new("beatmap_kind"),
                        BeatmapKind::iter(),
                    ))
                    .col(integer_null(Beatmap::DLCPack))
                    .col(json_binary(Beatmap::BPMRange))
                    .col(json_binary_null(Beatmap::Key4))
                    .col(json_binary_null(Beatmap::Key6))
                    .col(json_binary_null(Beatmap::Key8))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        type BeatmapModel = database::entities::beatmap::ActiveModel;
        type EBeatmapKind = database::entities::sea_orm_active_enums::BeatmapKind;

        fn new_beatmap(
            id: i32,
            name: &str,
            quality: i32,
            composer: &str,
            kind: EBeatmapKind,
            dlc_pack: Option<i32>,
            bpm_range: BPMRange,
            key4: BeatmapDifficulty,
            key6: BeatmapDifficulty,
            key8: BeatmapDifficulty,
        ) -> BeatmapModel {
            BeatmapModel {
                id: Set(id),
                name: Set(name.into()),
                quality: Set(quality),
                composer: Set(composer.into()),
                kind: Set(kind),
                dlc_pack: Set(dlc_pack),
                bpm_range: Set(serde_json::to_value(bpm_range).unwrap()),
                key4: Set(Some(serde_json::to_value(key4).unwrap())),
                key6: Set(Some(serde_json::to_value(key6).unwrap())),
                key8: Set(Some(serde_json::to_value(key8).unwrap())),
            }
        }

        /*
        In case anyone is wondering, I did not type that out, I wrote code to generate the bellow code.
        I am not a masochist.

        ```javascript
        console.log(songs.map(s => `new_beatmap(${s.id}, ${JSON.stringify(s.name)}, ${s.quality}, ${JSON.stringify(s.composer)}, eBeatmapKind::${['Tutorial','Free','Sale','Dlc'][s.songType]}, ${s.param > 0 ? 'Some('+s.param+')' : 'None'}, BPMRange { min: ${s.minBPM}, max: ${s.maxBPM} }, BeatmapDifficulty::new(${s.key4_easy_diff}, ${s.key4_easy_note}, ${s.key4_normal_diff}, ${s.key4_normal_note}, ${s.key4_hard_diff}, ${s.key4_hard_note}), BeatmapDifficulty::new(${s.key6_easy_diff}, ${s.key6_easy_note}, ${s.key6_normal_diff}, ${s.key6_normal_note}, ${s.key6_hard_diff}, ${s.key6_hard_note}), BeatmapDifficulty::new(${s.key8_easy_diff}, ${s.key8_easy_note}, ${s.key8_normal_diff}, ${s.key8_normal_note}, ${s.key8_hard_diff}, ${s.key8_hard_note}))`).join(',\n'))
        ```
        */

        #[rustfmt::skip]
        let beatmaps: Vec<BeatmapModel> = vec![
            new_beatmap(20000, "Tutorial: 1-1", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 135, max: 135 }, BeatmapDifficulty::new(0, 52, 0, 52, 0, 0), BeatmapDifficulty::new(0, 52, 0, 52, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 52)),
            new_beatmap(20001, "Tutorial: 1-2", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 90, max: 90 }, BeatmapDifficulty::new(0, 16, 0, 16, 0, 0), BeatmapDifficulty::new(0, 16, 0, 16, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 16)),
            new_beatmap(20002, "Tutorial: 1-3", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 117, max: 140 }, BeatmapDifficulty::new(0, 93, 0, 93, 0, 0), BeatmapDifficulty::new(0, 93, 0, 93, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 93)),
            new_beatmap(20003, "Tutorial: 1-4", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 117, max: 140 }, BeatmapDifficulty::new(0, 75, 0, 75, 0, 0), BeatmapDifficulty::new(0, 75, 0, 75, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 75)),
            new_beatmap(20005, "Tutorial: 1-5", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 175, max: 175 }, BeatmapDifficulty::new(0, 70, 0, 70, 0, 0), BeatmapDifficulty::new(0, 70, 0, 70, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 70)),
            new_beatmap(20006, "Tutorial: 2-1", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 135, max: 135 }, BeatmapDifficulty::new(0, 60, 0, 60, 0, 0), BeatmapDifficulty::new(0, 60, 0, 60, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 60)),
            new_beatmap(20007, "Tutorial: 2-2", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 135, max: 135 }, BeatmapDifficulty::new(0, 86, 0, 86, 0, 0), BeatmapDifficulty::new(0, 86, 0, 86, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 86)),
            new_beatmap(20008, "Tutorial: 2-3", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 145, max: 145 }, BeatmapDifficulty::new(0, 130, 0, 130, 0, 0), BeatmapDifficulty::new(0, 130, 0, 130, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 130)),
            new_beatmap(20009, "Tutorial: 2-4", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 145, max: 145 }, BeatmapDifficulty::new(0, 212, 0, 212, 0, 0), BeatmapDifficulty::new(0, 212, 0, 212, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 212)),
            new_beatmap(20010, "Tutorial: 2-5", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 98, max: 98 }, BeatmapDifficulty::new(0, 65, 0, 65, 0, 0), BeatmapDifficulty::new(0, 65, 0, 65, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 65)),
            new_beatmap(20011, "Tutorial: 3-1", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 140, max: 140 }, BeatmapDifficulty::new(0, 81, 0, 81, 0, 0), BeatmapDifficulty::new(0, 81, 0, 81, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 81)),
            new_beatmap(20012, "Tutorial: 3-2", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 140, max: 140 }, BeatmapDifficulty::new(0, 91, 0, 91, 0, 0), BeatmapDifficulty::new(0, 91, 0, 91, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 91)),
            new_beatmap(20013, "Tutorial: 3-3", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 130, max: 173 }, BeatmapDifficulty::new(0, 85, 0, 85, 0, 0), BeatmapDifficulty::new(0, 85, 0, 85, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 85)),
            new_beatmap(20014, "Tutorial: 3-4", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 130, max: 173 }, BeatmapDifficulty::new(0, 105, 0, 105, 0, 0), BeatmapDifficulty::new(0, 105, 0, 105, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 105)),
            new_beatmap(20015, "Tutorial: 3-5", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 130, max: 130 }, BeatmapDifficulty::new(0, 66, 0, 66, 0, 0), BeatmapDifficulty::new(0, 66, 0, 66, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 66)),
            new_beatmap(62001, "Offshore", 0, "Nhato", EBeatmapKind::Tutorial, None, BPMRange { min: 132, max: 132 }, BeatmapDifficulty::new(4, 609, 7, 847, 0, 0), BeatmapDifficulty::new(4, 609, 7, 847, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 1063)),
            new_beatmap(62003, "Rebirth", 0, "haloweak", EBeatmapKind::Tutorial, None, BPMRange { min: 138, max: 138 }, BeatmapDifficulty::new(3, 572, 7, 906, 0, 0), BeatmapDifficulty::new(3, 572, 7, 906, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 1144)),
            new_beatmap(62004, "Code Paradiso", 0, "himmel", EBeatmapKind::Free, None, BPMRange { min: 150, max: 150 }, BeatmapDifficulty::new(4, 369, 7, 725, 10, 862), BeatmapDifficulty::new(6, 610, 10, 923, 13, 948), BeatmapDifficulty::new(0, 0, 0, 0, 10, 782)),
            new_beatmap(62005, "Ice Cream", 0, "sky_delta feat. あさちる", EBeatmapKind::Sale, None, BPMRange { min: 130, max: 173 }, BeatmapDifficulty::new(3, 386, 5, 634, 8, 843), BeatmapDifficulty::new(4, 613, 6, 654, 11, 914), BeatmapDifficulty::new(0, 0, 0, 0, 9, 743)),
            new_beatmap(62006, "Dreamrainer", 0, "天遊", EBeatmapKind::Sale, None, BPMRange { min: 136, max: 136 }, BeatmapDifficulty::new(2, 354, 5, 771, 10, 1104), BeatmapDifficulty::new(3, 575, 6, 700, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 875)),
            new_beatmap(62007, "Frozen Paradise", 0, "GhostFinal", EBeatmapKind::Tutorial, None, BPMRange { min: 146, max: 146 }, BeatmapDifficulty::new(3, 392, 7, 540, 0, 0), BeatmapDifficulty::new(6, 392, 7, 540, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 626)),
            new_beatmap(62008, "Zorn", 0, "iKz", EBeatmapKind::Free, None, BPMRange { min: 150, max: 150 }, BeatmapDifficulty::new(3, 338, 7, 679, 0, 0), BeatmapDifficulty::new(5, 507, 8, 688, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 747)),
            new_beatmap(62010, "Empyrean", 0, "himmel", EBeatmapKind::Free, None, BPMRange { min: 175, max: 175 }, BeatmapDifficulty::new(3, 388, 8, 846, 13, 1193), BeatmapDifficulty::new(6, 606, 9, 846, 14, 1090), BeatmapDifficulty::new(0, 0, 0, 0, 10, 844)),
            new_beatmap(62011, "霜爆", 0, "himmel", EBeatmapKind::Free, None, BPMRange { min: 145, max: 145 }, BeatmapDifficulty::new(3, 367, 8, 1013, 10, 1050), BeatmapDifficulty::new(6, 786, 9, 1016, 12, 1114), BeatmapDifficulty::new(0, 0, 0, 0, 11, 1024)),
            new_beatmap(62012, "Helios", 0, "Akira Complex", EBeatmapKind::Free, None, BPMRange { min: 142, max: 142 }, BeatmapDifficulty::new(4, 683, 11, 1401, 13, 1579), BeatmapDifficulty::new(10, 1194, 11, 1445, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 12, 1322)),
            new_beatmap(62013, "Stairway to heaven", 0, "月代彩", EBeatmapKind::Free, None, BPMRange { min: 152, max: 152 }, BeatmapDifficulty::new(4, 398, 10, 1007, 14, 1312), BeatmapDifficulty::new(6, 751, 11, 1007, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 1076)),
            new_beatmap(62016, "The Formula", 0, "Junk", EBeatmapKind::Free, None, BPMRange { min: 144, max: 144 }, BeatmapDifficulty::new(4, 530, 7, 976, 0, 0), BeatmapDifficulty::new(7, 912, 9, 1011, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 1114)),
            new_beatmap(62017, "* Crow Solace *", 0, "m108", EBeatmapKind::Free, None, BPMRange { min: 188, max: 188 }, BeatmapDifficulty::new(5, 628, 12, 1229, 16, 1879), BeatmapDifficulty::new(7, 799, 13, 1274, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 14, 1312)),
            new_beatmap(62018, "Aerialwalker", 0, "天遊", EBeatmapKind::Free, None, BPMRange { min: 128, max: 128 }, BeatmapDifficulty::new(3, 440, 6, 758, 10, 942), BeatmapDifficulty::new(6, 682, 8, 766, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 821)),
            new_beatmap(62019, "Stage 5", 0, "LV.4", EBeatmapKind::Free, None, BPMRange { min: 145, max: 145 }, BeatmapDifficulty::new(4, 473, 4, 1098, 0, 0), BeatmapDifficulty::new(4, 903, 4, 1100, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 4, 1160)),
            new_beatmap(62020, "Right Now!", 0, "KO3", EBeatmapKind::Free, None, BPMRange { min: 150, max: 150 }, BeatmapDifficulty::new(3, 309, 7, 918, 0, 0), BeatmapDifficulty::new(5, 585, 8, 920, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 920)),
            new_beatmap(62021, "CUBICSPHERE", 0, "sky_delta", EBeatmapKind::Free, None, BPMRange { min: 179, max: 179 }, BeatmapDifficulty::new(6, 591, 10, 859, 0, 0), BeatmapDifficulty::new(8, 807, 10, 856, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 13, 971)),
            new_beatmap(62022, "Seedy Try", 0, "t+pazolite", EBeatmapKind::Free, None, BPMRange { min: 210, max: 210 }, BeatmapDifficulty::new(4, 414, 8, 814, 14, 1482), BeatmapDifficulty::new(7, 662, 10, 814, 16, 1643), BeatmapDifficulty::new(0, 0, 0, 0, 11, 832)),
            new_beatmap(62023, "Reborn", 0, "Scarfaith", EBeatmapKind::Free, None, BPMRange { min: 129, max: 129 }, BeatmapDifficulty::new(3, 352, 99, 565, 12, 871), BeatmapDifficulty::new(6, 511, 99, 565, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 624)),
            new_beatmap(62024, "White Light", 0, "KO3", EBeatmapKind::Free, None, BPMRange { min: 138, max: 138 }, BeatmapDifficulty::new(3, 317, 6, 534, 0, 0), BeatmapDifficulty::new(5, 546, 7, 556, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 553)),
            new_beatmap(62025, "普通DISCO", 0, "ilem", EBeatmapKind::Free, None, BPMRange { min: 110, max: 110 }, BeatmapDifficulty::new(2, 274, 7, 651, 0, 0), BeatmapDifficulty::new(3, 301, 10, 928, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 804)),
            new_beatmap(63001, "Wings of Liberty", 0, "himmel", EBeatmapKind::Free, None, BPMRange { min: 180, max: 180 }, BeatmapDifficulty::new(4, 398, 7, 809, 13, 1078), BeatmapDifficulty::new(5, 535, 8, 809, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 808)),
            new_beatmap(63002, "Scarlet Drop", 0, "人形兔 feat.樂正綾", EBeatmapKind::Tutorial, None, BPMRange { min: 240, max: 240 }, BeatmapDifficulty::new(4, 222, 8, 571, 0, 0), BeatmapDifficulty::new(4, 222, 8, 571, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 799)),
            new_beatmap(63003, "Sink into Nightmare", 0, "LEMiao feat.瑤山百靈", EBeatmapKind::Free, None, BPMRange { min: 145, max: 145 }, BeatmapDifficulty::new(3, 427, 7, 874, 10, 1117), BeatmapDifficulty::new(5, 634, 8, 888, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 885)),
            new_beatmap(63004, "造夢器", 0, "戰場原妖精 feat.洛天依", EBeatmapKind::Free, None, BPMRange { min: 158, max: 158 }, BeatmapDifficulty::new(3, 457, 11, 1062, 0, 0), BeatmapDifficulty::new(6, 796, 12, 1570, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 1267)),
            new_beatmap(63101, "Connected (Gammer Remix)", 0, "Akira Complex x Hommarju", EBeatmapKind::Free, None, BPMRange { min: 85, max: 85 }, BeatmapDifficulty::new(1, 141, 4, 588, 0, 0), BeatmapDifficulty::new(2, 250, 5, 589, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 8, 587)),
            new_beatmap(63102, "Connected (WRLD Remix)", 0, "Akira Complex x Hommarju", EBeatmapKind::Free, None, BPMRange { min: 85, max: 85 }, BeatmapDifficulty::new(3, 247, 5, 494, 0, 0), BeatmapDifficulty::new(4, 447, 7, 498, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 502)),
            new_beatmap(63103, "Connected(Zyon Ver)", 0, "Akira Complex x Hommarju", EBeatmapKind::Sale, None, BPMRange { min: 190, max: 190 }, BeatmapDifficulty::new(3, 481, 6, 947, 13, 1632), BeatmapDifficulty::new(3, 749, 7, 957, 14, 1632), BeatmapDifficulty::new(0, 0, 0, 0, 8, 919)),
            new_beatmap(63120, "Duet(From Hachihachi)", 0, "Skytree", EBeatmapKind::Free, None, BPMRange { min: 94, max: 122 }, BeatmapDifficulty::new(1, 245, 4, 552, 7, 704), BeatmapDifficulty::new(3, 422, 5, 552, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 6, 627)),
            new_beatmap(63121, "春菊(From Dynamix)", 0, "SUWAKI", EBeatmapKind::Free, None, BPMRange { min: 171, max: 171 }, BeatmapDifficulty::new(3, 428, 7, 740, 11, 1120), BeatmapDifficulty::new(5, 587, 8, 746, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 740)),
            new_beatmap(63122, "雷切 (From Hachihachi)", 0, "ICBM", EBeatmapKind::Sale, None, BPMRange { min: 152, max: 152 }, BeatmapDifficulty::new(2, 299, 6, 699, 13, 1311), BeatmapDifficulty::new(5, 447, 7, 715, 15, 1242), BeatmapDifficulty::new(0, 0, 0, 0, 10, 706)),
            new_beatmap(63123, "名無しの宣教師 (From Dynamix)", 0, "MIssionary", EBeatmapKind::Sale, None, BPMRange { min: 195, max: 195 }, BeatmapDifficulty::new(3, 367, 6, 718, 14, 1488), BeatmapDifficulty::new(4, 567, 7, 828, 14, 1335), BeatmapDifficulty::new(0, 0, 0, 0, 9, 828)),
            new_beatmap(63204, "Perspicuus_Aestus (From Dynamix)", 0, "Ryusse", EBeatmapKind::Sale, None, BPMRange { min: 185, max: 185 }, BeatmapDifficulty::new(3, 401, 11, 959, 15, 1242), BeatmapDifficulty::new(6, 505, 8, 671, 15, 1262), BeatmapDifficulty::new(0, 0, 0, 0, 10, 737)),
            new_beatmap(64001, "六道輪回", 0, "feitie", EBeatmapKind::Free, None, BPMRange { min: 160, max: 160 }, BeatmapDifficulty::new(3, 563, 8, 878, 0, 0), BeatmapDifficulty::new(6, 783, 10, 880, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 885)),
            new_beatmap(64002, "The Eighth Sin", 0, "MIssionary", EBeatmapKind::Free, None, BPMRange { min: 200, max: 200 }, BeatmapDifficulty::new(4, 349, 10, 841, 0, 0), BeatmapDifficulty::new(7, 534, 12, 845, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 12, 825)),
            new_beatmap(64003, "Incontrollable Desire", 0, "希望索任合資", EBeatmapKind::Free, None, BPMRange { min: 180, max: 180 }, BeatmapDifficulty::new(4, 488, 7, 746, 0, 0), BeatmapDifficulty::new(4, 680, 7, 745, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 754)),
            new_beatmap(64004, "伯爵&妖精", 0, "埋葬", EBeatmapKind::Free, None, BPMRange { min: 138, max: 138 }, BeatmapDifficulty::new(3, 408, 6, 773, 11, 1258), BeatmapDifficulty::new(5, 700, 7, 798, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 843)),
            new_beatmap(64005, "Kronos", 0, "sakuzyo", EBeatmapKind::Sale, None, BPMRange { min: 156, max: 156 }, BeatmapDifficulty::new(4, 538, 6, 653, 9, 939), BeatmapDifficulty::new(5, 666, 7, 879, 14, 1178), BeatmapDifficulty::new(0, 0, 0, 0, 10, 903)),
            new_beatmap(65004, "茉莉花的音符", 0, "谢谢P feat. 洛天依", EBeatmapKind::Tutorial, None, BPMRange { min: 180, max: 180 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(65006, "Mint Julep Chocolate", 0, "Zoey feat. komine", EBeatmapKind::Free, None, BPMRange { min: 130, max: 130 }, BeatmapDifficulty::new(1, 164, 3, 283, 0, 0), BeatmapDifficulty::new(2, 278, 3, 283, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 6, 386)),
            new_beatmap(65011, "吉原ラメント", 0, "亜沙 feat. MAYU", EBeatmapKind::Free, None, BPMRange { min: 195, max: 195 }, BeatmapDifficulty::new(3, 366, 6, 837, 10, 1192), BeatmapDifficulty::new(5, 598, 7, 836, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 934)),
            new_beatmap(65012, "炉心融解", 0, "iroha(sasaki) feat. MAYU", EBeatmapKind::Free, None, BPMRange { min: 165, max: 165 }, BeatmapDifficulty::new(4, 391, 7, 863, 9, 858), BeatmapDifficulty::new(5, 665, 8, 905, 11, 1110), BeatmapDifficulty::new(0, 0, 0, 0, 10, 871)),
            new_beatmap(65014, "一途な片思い、実らせたい小さな幸せ。", 0, "うたたP feat. MAYU", EBeatmapKind::Free, Some(2), BPMRange { min: 176, max: 176 }, BeatmapDifficulty::new(4, 452, 7, 678, 0, 0), BeatmapDifficulty::new(4, 529, 7, 679, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 687)),
            new_beatmap(65015, "え？あぁ、そう。", 0, "蝶々P feat. MAYU", EBeatmapKind::Tutorial, None, BPMRange { min: 192, max: 192 }, BeatmapDifficulty::new(5, 658, 8, 982, 0, 0), BeatmapDifficulty::new(5, 658, 8, 982, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 1278)),
            new_beatmap(65016, "青", 0, "164 feat. MAYU", EBeatmapKind::Tutorial, None, BPMRange { min: 90, max: 90 }, BeatmapDifficulty::new(3, 256, 6, 426, 0, 0), BeatmapDifficulty::new(3, 256, 6, 426, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 8, 530)),
            new_beatmap(65030, "Philosophy", 0, "Radianth", EBeatmapKind::Tutorial, None, BPMRange { min: 138, max: 138 }, BeatmapDifficulty::new(3, 744, 7, 1053, 0, 0), BeatmapDifficulty::new(3, 744, 7, 1053, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 1201)),
            new_beatmap(65031, "Urban Transit", 0, "mossari", EBeatmapKind::Free, None, BPMRange { min: 140, max: 140 }, BeatmapDifficulty::new(3, 239, 6, 556, 11, 786), BeatmapDifficulty::new(4, 350, 7, 556, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 574)),
            new_beatmap(65032, "赫", 0, "Blacklolita", EBeatmapKind::Free, None, BPMRange { min: 150, max: 150 }, BeatmapDifficulty::new(2, 274, 9, 1033, 0, 0), BeatmapDifficulty::new(5, 399, 99, 1101, 17, 2203), BeatmapDifficulty::new(0, 0, 0, 0, 10, 940)),
            new_beatmap(65033, "Adventure", 0, "S2NOISE", EBeatmapKind::Free, None, BPMRange { min: 110, max: 110 }, BeatmapDifficulty::new(3, 719, 10, 1299, 0, 0), BeatmapDifficulty::new(5, 921, 11, 1663, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 13, 1647)),
            new_beatmap(65034, "My Sweet Kiss", 0, "3R2 feat.瑶山百霊", EBeatmapKind::Free, None, BPMRange { min: 132, max: 132 }, BeatmapDifficulty::new(2, 296, 5, 520, 8, 747), BeatmapDifficulty::new(4, 520, 6, 723, 9, 987), BeatmapDifficulty::new(0, 0, 0, 0, 10, 853)),
            new_beatmap(65035, "Illumination", 0, "You feat.nayuta", EBeatmapKind::Free, None, BPMRange { min: 175, max: 175 }, BeatmapDifficulty::new(3, 303, 6, 748, 9, 746), BeatmapDifficulty::new(4, 562, 7, 768, 13, 1068), BeatmapDifficulty::new(0, 0, 0, 0, 9, 768)),
            new_beatmap(65036, "野史", 0, "Killerblood", EBeatmapKind::Sale, None, BPMRange { min: 100, max: 100 }, BeatmapDifficulty::new(3, 239, 5, 500, 10, 839), BeatmapDifficulty::new(3, 424, 7, 515, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 8, 516)),
            new_beatmap(65037, "I can reach you", 0, "TE.TOY", EBeatmapKind::Free, None, BPMRange { min: 105, max: 105 }, BeatmapDifficulty::new(2, 404, 5, 638, 9, 929), BeatmapDifficulty::new(3, 540, 6, 675, 9, 995), BeatmapDifficulty::new(0, 0, 0, 0, 9, 742)),
            new_beatmap(65100, "枯レ語リ", 0, "Ryuu feat. 流月", EBeatmapKind::Tutorial, None, BPMRange { min: 158, max: 158 }, BeatmapDifficulty::new(4, 528, 6, 739, 0, 0), BeatmapDifficulty::new(4, 528, 6, 739, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 8, 947)),
            new_beatmap(65101, "星空の花束", 0, "himmel feat. 瑶山百霊", EBeatmapKind::Free, None, BPMRange { min: 118, max: 118 }, BeatmapDifficulty::new(2, 239, 4, 425, 8, 744), BeatmapDifficulty::new(3, 376, 6, 433, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 8, 452)),
            new_beatmap(65102, "花语：玫瑰", 0, "himmel", EBeatmapKind::Free, None, BPMRange { min: 175, max: 175 }, BeatmapDifficulty::new(6, 413, 11, 980, 0, 0), BeatmapDifficulty::new(6, 619, 12, 1032, 14, 1251), BeatmapDifficulty::new(0, 0, 0, 0, 12, 1004)),
            new_beatmap(66001, "Vanish of Sakura", 0, "himmel", EBeatmapKind::Free, None, BPMRange { min: 140, max: 140 }, BeatmapDifficulty::new(3, 337, 7, 643, 11, 962), BeatmapDifficulty::new(5, 540, 8, 647, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 670)),
            new_beatmap(67001, "Maple Wind Acoustic Ver.", 0, "himmel", EBeatmapKind::Free, None, BPMRange { min: 135, max: 135 }, BeatmapDifficulty::new(4, 406, 8, 636, 0, 0), BeatmapDifficulty::new(6, 430, 9, 759, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 7, 389)),
            new_beatmap(67002, "戰歿者谷", 0, "Triodust", EBeatmapKind::Free, None, BPMRange { min: 100, max: 100 }, BeatmapDifficulty::new(2, 155, 5, 310, 0, 0), BeatmapDifficulty::new(2, 244, 5, 310, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 8, 310)),
            new_beatmap(67003, "Once a Dreamer", 0, "Shi Kuang Lee", EBeatmapKind::Free, None, BPMRange { min: 90, max: 90 }, BeatmapDifficulty::new(2, 322, 6, 533, 0, 0), BeatmapDifficulty::new(4, 470, 7, 533, 12, 1380), BeatmapDifficulty::new(0, 0, 0, 0, 8, 558)),
            new_beatmap(67004, "New Generation", 0, "John Towse", EBeatmapKind::Free, None, BPMRange { min: 190, max: 190 }, BeatmapDifficulty::new(4, 491, 8, 865, 0, 0), BeatmapDifficulty::new(7, 755, 9, 934, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 947)),
            new_beatmap(68001, "Alice Rainbow", 0, "金鎮", EBeatmapKind::Sale, None, BPMRange { min: 150, max: 150 }, BeatmapDifficulty::new(3, 384, 6, 557, 8, 910), BeatmapDifficulty::new(6, 543, 8, 722, 11, 912), BeatmapDifficulty::new(0, 0, 0, 0, 10, 707)),
            new_beatmap(68002, "Sakura Fubuki", 0, "Street", EBeatmapKind::Sale, None, BPMRange { min: 175, max: 175 }, BeatmapDifficulty::new(3, 210, 8, 461, 12, 975), BeatmapDifficulty::new(6, 457, 10, 489, 13, 930), BeatmapDifficulty::new(0, 0, 0, 0, 11, 472)),
            new_beatmap(68003, "Data Err0r", 0, "Lunatic Sounds", EBeatmapKind::Free, None, BPMRange { min: 180, max: 180 }, BeatmapDifficulty::new(3, 238, 8, 723, 12, 933), BeatmapDifficulty::new(8, 688, 9, 748, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 739)),
            new_beatmap(68004, "White Note", 0, "ARForest", EBeatmapKind::Free, None, BPMRange { min: 191, max: 191 }, BeatmapDifficulty::new(6, 440, 9, 791, 15, 1266), BeatmapDifficulty::new(7, 636, 9, 803, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 797)),
            new_beatmap(68005, "ラーメンのうた", 0, "ASACHILL prod. sky_delta", EBeatmapKind::Free, None, BPMRange { min: 131, max: 131 }, BeatmapDifficulty::new(3, 266, 6, 477, 7, 441), BeatmapDifficulty::new(6, 448, 8, 477, 10, 601), BeatmapDifficulty::new(0, 0, 0, 0, 9, 472)),
            new_beatmap(68006, "Altersist", 0, "Void", EBeatmapKind::Free, None, BPMRange { min: 138, max: 138 }, BeatmapDifficulty::new(4, 566, 8, 1011, 12, 1343), BeatmapDifficulty::new(8, 959, 10, 1081, 16, 1855), BeatmapDifficulty::new(0, 0, 0, 0, 11, 1114)),
            new_beatmap(68007, "Vector Cannon", 0, "Akira Complex", EBeatmapKind::Free, None, BPMRange { min: 105, max: 105 }, BeatmapDifficulty::new(4, 500, 7, 626, 0, 0), BeatmapDifficulty::new(7, 603, 8, 733, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 761)),
            new_beatmap(68008, "Primum Mobile", 0, "himmel", EBeatmapKind::Sale, None, BPMRange { min: 175, max: 175 }, BeatmapDifficulty::new(4, 409, 8, 767, 13, 1355), BeatmapDifficulty::new(6, 477, 10, 929, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 13, 855)),
            new_beatmap(68009, "Razor", 0, "Zekk", EBeatmapKind::Free, None, BPMRange { min: 128, max: 128 }, BeatmapDifficulty::new(4, 600, 8, 927, 0, 0), BeatmapDifficulty::new(6, 808, 10, 965, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 12, 1028)),
            new_beatmap(68101, "Gone with the Beat", 0, "typeMARS", EBeatmapKind::Free, None, BPMRange { min: 176, max: 176 }, BeatmapDifficulty::new(4, 389, 9, 903, 16, 1280), BeatmapDifficulty::new(7, 677, 10, 903, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 922)),
            new_beatmap(68102, "Stardust Overdrive", 0, "typeMARS", EBeatmapKind::Free, None, BPMRange { min: 175, max: 175 }, BeatmapDifficulty::new(5, 455, 9, 919, 0, 0), BeatmapDifficulty::new(7, 745, 10, 932, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 12, 922)),
            new_beatmap(68103, "Triumph&Regret", 0, "typeMARS", EBeatmapKind::Free, None, BPMRange { min: 175, max: 175 }, BeatmapDifficulty::new(5, 467, 10, 986, 0, 0), BeatmapDifficulty::new(8, 890, 12, 1034, 17, 1854), BeatmapDifficulty::new(0, 0, 0, 0, 14, 1040)),
            new_beatmap(68104, "Utopia", 0, "typeMARS", EBeatmapKind::Free, None, BPMRange { min: 150, max: 150 }, BeatmapDifficulty::new(3, 408, 8, 874, 0, 0), BeatmapDifficulty::new(6, 688, 9, 873, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 894)),
            new_beatmap(68105, "流れ星に願うよいま", 0, "LemonWhisper(EBICO & ノゾム)", EBeatmapKind::Free, None, BPMRange { min: 128, max: 128 }, BeatmapDifficulty::new(2, 298, 5, 701, 0, 0), BeatmapDifficulty::new(4, 545, 7, 702, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 8, 702)),
            new_beatmap(68106, "Persephone", 0, "EBICO", EBeatmapKind::Free, None, BPMRange { min: 185, max: 185 }, BeatmapDifficulty::new(8, 796, 15, 1578, 16, 1698), BeatmapDifficulty::new(8, 796, 14, 1367, 18, 2263), BeatmapDifficulty::new(0, 0, 0, 0, 17, 1449)),
            new_beatmap(68107, "Vision of Eaux", 0, "月代彩", EBeatmapKind::Free, None, BPMRange { min: 150, max: 150 }, BeatmapDifficulty::new(3, 345, 7, 825, 12, 1108), BeatmapDifficulty::new(5, 521, 8, 831, 13, 1153), BeatmapDifficulty::new(0, 0, 0, 0, 10, 871)),
            new_beatmap(68108, "Nirvana", 0, "Zekk", EBeatmapKind::Sale, None, BPMRange { min: 170, max: 170 }, BeatmapDifficulty::new(3, 306, 6, 650, 14, 1399), BeatmapDifficulty::new(6, 504, 7, 656, 15, 1445), BeatmapDifficulty::new(0, 0, 0, 0, 9, 684)),
            new_beatmap(69001, "それから", 0, "noVaTion", EBeatmapKind::Free, None, BPMRange { min: 96, max: 96 }, BeatmapDifficulty::new(2, 214, 5, 468, 7, 601), BeatmapDifficulty::new(3, 316, 6, 546, 8, 751), BeatmapDifficulty::new(0, 0, 0, 0, 7, 566)),
            new_beatmap(69002, "远枫", 0, "himmel feat.瑤山百靈", EBeatmapKind::Free, None, BPMRange { min: 135, max: 135 }, BeatmapDifficulty::new(2, 206, 5, 419, 6, 522), BeatmapDifficulty::new(4, 343, 6, 419, 10, 872), BeatmapDifficulty::new(0, 0, 0, 0, 7, 419)),
            new_beatmap(69008, "Cryonix", 0, "sky_delta", EBeatmapKind::Sale, None, BPMRange { min: 184, max: 184 }, BeatmapDifficulty::new(3, 335, 8, 741, 99, 1444), BeatmapDifficulty::new(5, 505, 9, 769, 17, 1165), BeatmapDifficulty::new(0, 0, 0, 0, 12, 780)),
            new_beatmap(69009, "StartingLine", 0, "G.K feat.洛天依", EBeatmapKind::Tutorial, None, BPMRange { min: 132, max: 132 }, BeatmapDifficulty::new(4, 461, 7, 533, 0, 0), BeatmapDifficulty::new(4, 461, 7, 533, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 627)),
            new_beatmap(69012, "機械少女的日常", 0, "錄音P", EBeatmapKind::Free, None, BPMRange { min: 160, max: 160 }, BeatmapDifficulty::new(3, 318, 6, 729, 0, 0), BeatmapDifficulty::new(5, 633, 7, 733, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 740)),
            new_beatmap(69017, "Lantinid", 0, "ARForest", EBeatmapKind::Free, None, BPMRange { min: 202, max: 202 }, BeatmapDifficulty::new(4, 220, 10, 729, 16, 1171), BeatmapDifficulty::new(7, 580, 10, 744, 99, 1967), BeatmapDifficulty::new(0, 0, 0, 0, 12, 757)),
            new_beatmap(69018, "Reflectia", 0, "cnsouka", EBeatmapKind::Sale, None, BPMRange { min: 138, max: 138 }, BeatmapDifficulty::new(2, 395, 7, 705, 9, 1029), BeatmapDifficulty::new(7, 678, 8, 710, 11, 1128), BeatmapDifficulty::new(0, 0, 0, 0, 10, 742)),
            new_beatmap(69901, "功夫ガールズ", 0, "hoskey", EBeatmapKind::Free, None, BPMRange { min: 164, max: 164 }, BeatmapDifficulty::new(5, 494, 9, 861, 0, 0), BeatmapDifficulty::new(7, 726, 11, 894, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 895)),
            new_beatmap(69903, "Connected (かめりあ Remix)", 0, "Akira Complex x Hommarju", EBeatmapKind::Free, None, BPMRange { min: 150, max: 150 }, BeatmapDifficulty::new(4, 479, 7, 670, 11, 1173), BeatmapDifficulty::new(5, 593, 8, 670, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 670)),
            new_beatmap(80000, "See You", 0, "WRLD/Richard Caddock", EBeatmapKind::Free, None, BPMRange { min: 125, max: 125 }, BeatmapDifficulty::new(7, 506, 10, 793, 0, 0), BeatmapDifficulty::new(5, 413, 11, 971, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 811)),
            new_beatmap(80001, "Load of the Ciel", 0, "sky_delta vs Street", EBeatmapKind::Free, None, BPMRange { min: 175, max: 175 }, BeatmapDifficulty::new(4, 374, 99, 782, 99, 1285), BeatmapDifficulty::new(5, 628, 99, 916, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 99, 935)),
            new_beatmap(80002, "Grenade", 0, "sky_delta", EBeatmapKind::Sale, None, BPMRange { min: 181, max: 181 }, BeatmapDifficulty::new(5, 703, 9, 908, 0, 0), BeatmapDifficulty::new(6, 718, 10, 878, 16, 1677), BeatmapDifficulty::new(0, 0, 0, 0, 11, 1362)),
            new_beatmap(80003, "Check Point", 0, "Nitro Fun/Hyper Potions", EBeatmapKind::Free, None, BPMRange { min: 117, max: 140 }, BeatmapDifficulty::new(4, 529, 7, 846, 12, 1539), BeatmapDifficulty::new(5, 653, 8, 755, 13, 1579), BeatmapDifficulty::new(0, 0, 0, 0, 9, 757)),
            new_beatmap(80004, "Xintessence", 0, "紅葉月梛葉", EBeatmapKind::Free, None, BPMRange { min: 155, max: 155 }, BeatmapDifficulty::new(3, 530, 8, 854, 13, 1408), BeatmapDifficulty::new(5, 704, 9, 857, 13, 1425), BeatmapDifficulty::new(0, 0, 0, 0, 10, 915)),
            new_beatmap(80005, "Garden of Eden", 0, "Iris", EBeatmapKind::Free, None, BPMRange { min: 90, max: 90 }, BeatmapDifficulty::new(2, 311, 5, 444, 7, 782), BeatmapDifficulty::new(4, 476, 6, 458, 8, 785), BeatmapDifficulty::new(0, 0, 0, 0, 9, 455)),
            new_beatmap(80006, "Dolfie Dream", 0, "O2i3", EBeatmapKind::Free, None, BPMRange { min: 175, max: 175 }, BeatmapDifficulty::new(4, 385, 6, 776, 0, 0), BeatmapDifficulty::new(5, 496, 6, 698, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 696)),
            new_beatmap(80007, "HAPPY*FOREVER", 0, "monaclo*", EBeatmapKind::Sale, None, BPMRange { min: 209, max: 209 }, BeatmapDifficulty::new(4, 422, 7, 732, 99, 1174), BeatmapDifficulty::new(7, 701, 10, 883, 16, 1347), BeatmapDifficulty::new(0, 0, 0, 0, 11, 893)),
            new_beatmap(80008, "Paradigm Shift", 0, "モリモリあつし", EBeatmapKind::Sale, None, BPMRange { min: 145, max: 145 }, BeatmapDifficulty::new(4, 474, 9, 788, 14, 1209), BeatmapDifficulty::new(8, 688, 11, 856, 16, 1278), BeatmapDifficulty::new(0, 0, 0, 0, 12, 873)),
            new_beatmap(80009, "Summer Days", 0, "Ans*", EBeatmapKind::Sale, None, BPMRange { min: 186, max: 186 }, BeatmapDifficulty::new(4, 405, 6, 589, 14, 1635), BeatmapDifficulty::new(5, 590, 9, 789, 15, 1636), BeatmapDifficulty::new(0, 0, 0, 0, 10, 788)),
            new_beatmap(80010, "Cernunnos's Horn", 0, "Crimsona", EBeatmapKind::Sale, None, BPMRange { min: 150, max: 150 }, BeatmapDifficulty::new(5, 504, 7, 640, 11, 921), BeatmapDifficulty::new(6, 683, 8, 682, 11, 963), BeatmapDifficulty::new(0, 0, 0, 0, 10, 688)),
            new_beatmap(80011, "Grisaille", 0, "Nego_tiator", EBeatmapKind::Sale, None, BPMRange { min: 152, max: 152 }, BeatmapDifficulty::new(3, 284, 10, 930, 13, 1259), BeatmapDifficulty::new(5, 520, 11, 1008, 17, 1866), BeatmapDifficulty::new(0, 0, 0, 0, 12, 1079)),
            new_beatmap(80012, "Our Rusty Memories", 0, "Artificial Idol", EBeatmapKind::Sale, None, BPMRange { min: 100, max: 100 }, BeatmapDifficulty::new(1, 170, 4, 427, 0, 0), BeatmapDifficulty::new(3, 427, 5, 489, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 7, 489)),
            new_beatmap(80013, "Life Porter", 0, "すてらべえ", EBeatmapKind::Sale, None, BPMRange { min: 130, max: 130 }, BeatmapDifficulty::new(3, 317, 6, 442, 99, 1177), BeatmapDifficulty::new(5, 446, 8, 529, 16, 1259), BeatmapDifficulty::new(0, 0, 0, 0, 10, 529)),
            new_beatmap(80014, "My Story", 0, "Ryo Arue", EBeatmapKind::Sale, None, BPMRange { min: 175, max: 175 }, BeatmapDifficulty::new(3, 418, 6, 602, 99, 1403), BeatmapDifficulty::new(5, 570, 7, 868, 12, 1406), BeatmapDifficulty::new(0, 0, 0, 0, 10, 867)),
            new_beatmap(80015, "Stardust Highway", 0, "NeLiME", EBeatmapKind::Sale, None, BPMRange { min: 176, max: 176 }, BeatmapDifficulty::new(4, 462, 7, 700, 13, 1265), BeatmapDifficulty::new(7, 700, 8, 837, 17, 1907), BeatmapDifficulty::new(0, 0, 0, 0, 11, 859)),
            new_beatmap(80016, "Upon Ararat", 0, "Iris x VSX feat. LynH", EBeatmapKind::Free, None, BPMRange { min: 130, max: 130 }, BeatmapDifficulty::new(1, 314, 4, 421, 9, 992), BeatmapDifficulty::new(3, 421, 6, 749, 10, 1186), BeatmapDifficulty::new(0, 0, 0, 0, 9, 789)),
            new_beatmap(80017, "晴れた空", 0, "noVaTion", EBeatmapKind::Free, None, BPMRange { min: 98, max: 98 }, BeatmapDifficulty::new(2, 327, 5, 459, 0, 0), BeatmapDifficulty::new(4, 398, 5, 520, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 7, 508)),
            new_beatmap(80018, "Infection", 0, "A.I", EBeatmapKind::Sale, None, BPMRange { min: 145, max: 145 }, BeatmapDifficulty::new(4, 610, 7, 869, 13, 1660), BeatmapDifficulty::new(6, 869, 9, 996, 18, 2494), BeatmapDifficulty::new(0, 0, 0, 0, 11, 991)),
            new_beatmap(80019, "Fluctuation", 0, "Zekk", EBeatmapKind::Sale, None, BPMRange { min: 194, max: 194 }, BeatmapDifficulty::new(5, 459, 9, 868, 14, 1504), BeatmapDifficulty::new(7, 626, 10, 885, 17, 1363), BeatmapDifficulty::new(0, 0, 0, 0, 10, 869)),
            new_beatmap(80020, "夢見の少年", 0, "Aoi", EBeatmapKind::Sale, None, BPMRange { min: 240, max: 240 }, BeatmapDifficulty::new(5, 600, 10, 914, 99, 1408), BeatmapDifficulty::new(10, 962, 12, 1027, 17, 1379), BeatmapDifficulty::new(0, 0, 0, 0, 12, 1036)),
            new_beatmap(80021, "Hua Sui Yue", 0, "V.K克", EBeatmapKind::Free, None, BPMRange { min: 148, max: 148 }, BeatmapDifficulty::new(3, 451, 7, 776, 0, 0), BeatmapDifficulty::new(3, 495, 7, 908, 11, 1159), BeatmapDifficulty::new(0, 0, 0, 0, 9, 954)),
            new_beatmap(80023, "Entanglement", 0, "月代彩", EBeatmapKind::Free, None, BPMRange { min: 128, max: 128 }, BeatmapDifficulty::new(4, 556, 8, 1185, 13, 1486), BeatmapDifficulty::new(7, 1002, 10, 1216, 14, 1498), BeatmapDifficulty::new(0, 0, 0, 0, 12, 1340)),
            new_beatmap(80026, "滅衝クインテセンス", 0, "wa.", EBeatmapKind::Free, None, BPMRange { min: 190, max: 190 }, BeatmapDifficulty::new(5, 525, 11, 955, 0, 0), BeatmapDifficulty::new(7, 716, 11, 1079, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 1072)),
            new_beatmap(80027, "事象の地平線", 0, "wa.", EBeatmapKind::Free, None, BPMRange { min: 169, max: 169 }, BeatmapDifficulty::new(8, 701, 13, 1151, 15, 1281), BeatmapDifficulty::new(8, 616, 13, 1105, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 746)),
            new_beatmap(80028, "The World of Cyber", 0, "LATI", EBeatmapKind::Free, None, BPMRange { min: 135, max: 135 }, BeatmapDifficulty::new(4, 340, 8, 735, 0, 0), BeatmapDifficulty::new(6, 661, 9, 742, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 725)),
            new_beatmap(80029, "Lailah", 0, "隼", EBeatmapKind::Free, None, BPMRange { min: 173, max: 173 }, BeatmapDifficulty::new(5, 731, 8, 911, 0, 0), BeatmapDifficulty::new(6, 882, 12, 1555, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 12, 1561)),
            new_beatmap(80030, "Ooi", 0, "O2i3", EBeatmapKind::Free, None, BPMRange { min: 216, max: 216 }, BeatmapDifficulty::new(7, 712, 11, 1204, 16, 1545), BeatmapDifficulty::new(5, 584, 13, 1155, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 12, 1194)),
            new_beatmap(80031, "Full Control", 0, "RiraN", EBeatmapKind::Sale, None, BPMRange { min: 150, max: 150 }, BeatmapDifficulty::new(2, 226, 6, 641, 11, 960), BeatmapDifficulty::new(5, 506, 7, 639, 13, 1083), BeatmapDifficulty::new(0, 0, 0, 0, 10, 733)),
            new_beatmap(80032, "めくる", 0, "Ms.", EBeatmapKind::Free, None, BPMRange { min: 179, max: 179 }, BeatmapDifficulty::new(3, 420, 5, 626, 10, 887), BeatmapDifficulty::new(5, 591, 7, 726, 10, 860), BeatmapDifficulty::new(0, 0, 0, 0, 11, 813)),
            new_beatmap(80033, "ALDEBARAN", 0, "比嘉憲吾", EBeatmapKind::Free, None, BPMRange { min: 180, max: 180 }, BeatmapDifficulty::new(4, 358, 7, 608, 11, 862), BeatmapDifficulty::new(6, 552, 8, 636, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 744)),
            new_beatmap(80034, "Exalt", 0, "はがね&7mai", EBeatmapKind::Sale, None, BPMRange { min: 220, max: 220 }, BeatmapDifficulty::new(7, 717, 99, 1942, 17, 2395), BeatmapDifficulty::new(8, 964, 12, 1503, 16, 1997), BeatmapDifficulty::new(0, 0, 0, 0, 14, 1647)),
            new_beatmap(80035, "前人未闘", 0, "からとP", EBeatmapKind::Free, None, BPMRange { min: 125, max: 125 }, BeatmapDifficulty::new(2, 263, 7, 719, 10, 772), BeatmapDifficulty::new(5, 479, 10, 768, 11, 1057), BeatmapDifficulty::new(0, 0, 0, 0, 11, 810)),
            new_beatmap(80036, "天風", 0, "うまるつふり", EBeatmapKind::Free, None, BPMRange { min: 147, max: 147 }, BeatmapDifficulty::new(3, 471, 8, 981, 0, 0), BeatmapDifficulty::new(5, 792, 9, 983, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 989)),
            new_beatmap(80037, "Angelica", 0, "yusi x Awfuless", EBeatmapKind::Free, None, BPMRange { min: 190, max: 190 }, BeatmapDifficulty::new(7, 640, 11, 1025, 0, 0), BeatmapDifficulty::new(8, 853, 12, 1029, 16, 1626), BeatmapDifficulty::new(0, 0, 0, 0, 12, 939)),
            new_beatmap(80038, "Sweet Little Chocolate", 0, "uma", EBeatmapKind::Free, None, BPMRange { min: 170, max: 170 }, BeatmapDifficulty::new(4, 574, 11, 1224, 0, 0), BeatmapDifficulty::new(7, 1002, 10, 1224, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 13, 1223)),
            new_beatmap(80039, "Sakiko(Zyon Edit)", 0, "Kanata.S", EBeatmapKind::Free, None, BPMRange { min: 139, max: 139 }, BeatmapDifficulty::new(3, 247, 8, 774, 0, 0), BeatmapDifficulty::new(5, 518, 8, 774, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 12, 773)),
            new_beatmap(80040, "Uwabami", 0, "Kplecraft", EBeatmapKind::Free, None, BPMRange { min: 150, max: 150 }, BeatmapDifficulty::new(4, 653, 13, 1500, 0, 0), BeatmapDifficulty::new(10, 1215, 14, 1501, 16, 1748), BeatmapDifficulty::new(0, 0, 0, 0, 99, 1748)),
            new_beatmap(80041, "Veritas", 0, "Nhato/Taishi", EBeatmapKind::Free, None, BPMRange { min: 135, max: 135 }, BeatmapDifficulty::new(99, 545, 99, 1086, 12, 1964), BeatmapDifficulty::new(99, 917, 99, 1253, 15, 2704), BeatmapDifficulty::new(0, 0, 0, 0, 99, 1321)),
            new_beatmap(80042, "Elf", 0, "隼", EBeatmapKind::Free, None, BPMRange { min: 159, max: 159 }, BeatmapDifficulty::new(5, 704, 10, 1152, 0, 0), BeatmapDifficulty::new(7, 1114, 14, 1566, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 15, 1576)),
            new_beatmap(80043, "Never BEE", 0, "Papa", EBeatmapKind::Free, None, BPMRange { min: 140, max: 140 }, BeatmapDifficulty::new(3, 298, 7, 764, 10, 1036), BeatmapDifficulty::new(7, 755, 8, 769, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 914)),
            new_beatmap(80044, "明日には、消えてゆく  INVAXIONver.", 0, "KMA万華鏡音楽同盟", EBeatmapKind::Free, None, BPMRange { min: 140, max: 140 }, BeatmapDifficulty::new(3, 293, 6, 705, 0, 0), BeatmapDifficulty::new(5, 553, 7, 705, 10, 981), BeatmapDifficulty::new(0, 0, 0, 0, 9, 705)),
            new_beatmap(80045, "世界一のネコ姫さま  INVAXIONver.", 0, "KMA万華鏡音楽同盟", EBeatmapKind::Free, None, BPMRange { min: 154, max: 154 }, BeatmapDifficulty::new(4, 513, 7, 890, 0, 0), BeatmapDifficulty::new(5, 642, 8, 905, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 912)),
            new_beatmap(80046, "Evening Primrose", 0, "WyvernP", EBeatmapKind::Free, None, BPMRange { min: 100, max: 200 }, BeatmapDifficulty::new(6, 621, 9, 857, 0, 0), BeatmapDifficulty::new(7, 622, 10, 868, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 886)),
            new_beatmap(81001, "Fatal Sphere", 0, "A.Ki vs F4LSE", EBeatmapKind::Free, None, BPMRange { min: 172, max: 172 }, BeatmapDifficulty::new(6, 606, 12, 1220, 0, 0), BeatmapDifficulty::new(5, 496, 9, 773, 12, 1281), BeatmapDifficulty::new(0, 0, 0, 0, 9, 696)),
            new_beatmap(81002, "W-Inda", 0, "Se-U-Ra", EBeatmapKind::Free, None, BPMRange { min: 170, max: 170 }, BeatmapDifficulty::new(5, 516, 12, 1098, 0, 0), BeatmapDifficulty::new(6, 578, 11, 1034, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 915)),
            new_beatmap(81003, "天海原～あまみはら～", 0, "うまるつふり", EBeatmapKind::Free, None, BPMRange { min: 231, max: 231 }, BeatmapDifficulty::new(5, 504, 10, 908, 0, 0), BeatmapDifficulty::new(5, 596, 12, 865, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 12, 856)),
            new_beatmap(81004, "Zethiris (INVAXION Edit)", 0, "rareguy", EBeatmapKind::Free, None, BPMRange { min: 176, max: 176 }, BeatmapDifficulty::new(6, 613, 9, 854, 0, 0), BeatmapDifficulty::new(8, 822, 10, 988, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 11, 977)),
            new_beatmap(81005, "Asgard", 0, "SLT/Hain/rmk", EBeatmapKind::Free, None, BPMRange { min: 260, max: 260 }, BeatmapDifficulty::new(4, 552, 10, 1002, 0, 0), BeatmapDifficulty::new(5, 640, 10, 1021, 15, 1803), BeatmapDifficulty::new(0, 0, 0, 0, 11, 1003)),
            new_beatmap(81015, "Colorful Days♪", 0, "P4koo feat.つゆり花鈴", EBeatmapKind::Free, None, BPMRange { min: 136, max: 136 }, BeatmapDifficulty::new(3, 348, 11, 999, 0, 0), BeatmapDifficulty::new(4, 362, 9, 769, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 489)),
            new_beatmap(81101, "Evening Drive", 0, "aoWAVE", EBeatmapKind::Tutorial, None, BPMRange { min: 110, max: 110 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(81102, "Melusia", 0, "Se-U-Ra & seatrus", EBeatmapKind::Tutorial, None, BPMRange { min: 195, max: 195 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(81103, "Cybernetics", 0, "Jun Kuroda", EBeatmapKind::Tutorial, None, BPMRange { min: 170, max: 170 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(81104, "The True Verdict", 0, "siqlo vs. Aoi", EBeatmapKind::Tutorial, None, BPMRange { min: 174, max: 174 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(81105, "Hacktivism", 0, "Aoi vs. siqlo", EBeatmapKind::Tutorial, None, BPMRange { min: 165, max: 165 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(81106, "Afterlife", 0, "Neutral Moon / HyuN", EBeatmapKind::Free, None, BPMRange { min: 210, max: 210 }, BeatmapDifficulty::new(9, 800, 13, 1347, 18, 1781), BeatmapDifficulty::new(9, 844, 12, 1292, 19, 2248), BeatmapDifficulty::new(0, 0, 0, 0, 12, 1295)),
            new_beatmap(81107, "The Tyrant", 0, "surk", EBeatmapKind::Tutorial, None, BPMRange { min: 160, max: 160 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(81108, "Ginevra", 0, "MYUKKE.", EBeatmapKind::Tutorial, None, BPMRange { min: 170, max: 170 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(81109, "Immortal Palace", 0, "pan", EBeatmapKind::Free, None, BPMRange { min: 132, max: 132 }, BeatmapDifficulty::new(5, 504, 10, 882, 0, 0), BeatmapDifficulty::new(5, 477, 11, 990, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 12, 1084)),
            new_beatmap(81110, "Valmung", 0, "！すでのな", EBeatmapKind::Tutorial, None, BPMRange { min: 180, max: 222 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(82000, "乾杯 - (゜  -゜ )つロ", 0, "H.K.君", EBeatmapKind::Free, None, BPMRange { min: 90, max: 90 }, BeatmapDifficulty::new(22, 260, 33, 644, 0, 0), BeatmapDifficulty::new(22, 509, 33, 662, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 66, 685)),
            new_beatmap(82001, "交织 together", 0, "DMYoung feat.泠鸢yousa、hanser", EBeatmapKind::Free, None, BPMRange { min: 132, max: 132 }, BeatmapDifficulty::new(2, 229, 5, 620, 0, 0), BeatmapDifficulty::new(4, 504, 7, 624, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 674)),
            new_beatmap(82002, "与你同行 ~B With U~", 0, "kz（livetune） feat.22娘（幽舞越山）", EBeatmapKind::Free, None, BPMRange { min: 130, max: 130 }, BeatmapDifficulty::new(3, 338, 6, 672, 8, 728), BeatmapDifficulty::new(4, 573, 7, 681, 9, 972), BeatmapDifficulty::new(0, 0, 0, 0, 8, 763)),
            new_beatmap(82003, "流★群 Meteor Stream", 0, "H.K.君 with Cnsouka feat.GUMI", EBeatmapKind::Free, None, BPMRange { min: 125, max: 125 }, BeatmapDifficulty::new(2, 241, 5, 615, 0, 0), BeatmapDifficulty::new(4, 516, 7, 651, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 8, 681)),
            new_beatmap(82004, "新战场", 0, "DMYoung feat.茶理理", EBeatmapKind::Free, None, BPMRange { min: 185, max: 185 }, BeatmapDifficulty::new(6, 757, 11, 1204, 14, 1466), BeatmapDifficulty::new(7, 797, 11, 1231, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 13, 1243)),
            new_beatmap(82005, "勾指起誓", 0, "ilem", EBeatmapKind::Dlc, None, BPMRange { min: 85, max: 85 }, BeatmapDifficulty::new(1, 161, 5, 602, 8, 586), BeatmapDifficulty::new(4, 437, 6, 602, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 7, 638)),
            new_beatmap(82006, "夜间出租车", 0, "ilem", EBeatmapKind::Dlc, None, BPMRange { min: 105, max: 105 }, BeatmapDifficulty::new(1, 178, 4, 526, 0, 0), BeatmapDifficulty::new(3, 425, 4, 526, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 6, 526)),
            new_beatmap(82007, "大氿歌", 0, "ilem", EBeatmapKind::Dlc, None, BPMRange { min: 100, max: 100 }, BeatmapDifficulty::new(1, 226, 4, 539, 0, 0), BeatmapDifficulty::new(3, 413, 5, 555, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 7, 555)),
            new_beatmap(82008, "写给我家狗的歌", 0, "ilem", EBeatmapKind::Free, None, BPMRange { min: 95, max: 95 }, BeatmapDifficulty::new(1, 142, 3, 342, 0, 0), BeatmapDifficulty::new(2, 272, 4, 342, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 5, 320)),
            new_beatmap(82009, "售楼小姐", 0, "ilem", EBeatmapKind::Free, None, BPMRange { min: 108, max: 108 }, BeatmapDifficulty::new(1, 334, 6, 896, 0, 0), BeatmapDifficulty::new(3, 567, 7, 921, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 8, 875)),
            new_beatmap(82010, "威海", 0, "ilem", EBeatmapKind::Free, None, BPMRange { min: 100, max: 100 }, BeatmapDifficulty::new(1, 129, 3, 333, 0, 0), BeatmapDifficulty::new(2, 250, 4, 333, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 5, 315)),
            new_beatmap(82011, "得过且过的勇者", 0, "ilem", EBeatmapKind::Dlc, None, BPMRange { min: 105, max: 105 }, BeatmapDifficulty::new(2, 170, 5, 538, 0, 0), BeatmapDifficulty::new(4, 367, 6, 538, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 7, 540)),
            new_beatmap(82012, "lemon", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 87, max: 87 }, BeatmapDifficulty::new(1, 100, 1, 100, 0, 0), BeatmapDifficulty::new(1, 100, 1, 100, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 1, 100)),
            new_beatmap(82013, "loser", 0, "", EBeatmapKind::Tutorial, None, BPMRange { min: 121, max: 121 }, BeatmapDifficulty::new(1, 100, 1, 100, 0, 0), BeatmapDifficulty::new(1, 100, 1, 100, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 1, 100)),
            new_beatmap(84100, "Paramita", 0, "Papa", EBeatmapKind::Free, None, BPMRange { min: 140, max: 140 }, BeatmapDifficulty::new(0, 0, 0, 0, 99, 428), BeatmapDifficulty::new(0, 0, 0, 0, 99, 428), BeatmapDifficulty::new(0, 0, 0, 0, 99, 451)),
            new_beatmap(84101, "3pm.noitcefnI", 0, "xartauqa", EBeatmapKind::Free, None, BPMRange { min: 145, max: 145 }, BeatmapDifficulty::new(0, 0, 0, 0, 99, 1470), BeatmapDifficulty::new(0, 0, 0, 0, 99, 1455), BeatmapDifficulty::new(0, 0, 0, 0, 99, 1470)),
            new_beatmap(84102, "Bacon Omelet", 0, "Woolroll vs UN1C0DE", EBeatmapKind::Free, None, BPMRange { min: 140, max: 140 }, BeatmapDifficulty::new(5, 828, 12, 1513, 99, 1438), BeatmapDifficulty::new(8, 979, 14, 1665, 99, 1503), BeatmapDifficulty::new(0, 0, 0, 0, 99, 1411)),
            new_beatmap(90000, "星と花のささやき", 0, "铃科", EBeatmapKind::Free, None, BPMRange { min: 143, max: 143 }, BeatmapDifficulty::new(4, 606, 8, 1105, 0, 0), BeatmapDifficulty::new(4, 558, 8, 1043, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 9, 1012)),
            new_beatmap(90001, "黑色狂奏曲", 0, "埋葬", EBeatmapKind::Tutorial, None, BPMRange { min: 60, max: 166 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(90002, "花祭のリズム", 0, "埋葬", EBeatmapKind::Tutorial, None, BPMRange { min: 180, max: 180 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(90003, "开心病", 0, "埋葬", EBeatmapKind::Tutorial, None, BPMRange { min: 131, max: 131 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(90004, "风屿", 0, "埋葬", EBeatmapKind::Tutorial, None, BPMRange { min: 170, max: 170 }, BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 0, 0)),
            new_beatmap(90005, "万吨匿名信", 0, "埋葬", EBeatmapKind::Free, None, BPMRange { min: 120, max: 120 }, BeatmapDifficulty::new(6, 604, 11, 1224, 0, 0), BeatmapDifficulty::new(7, 634, 11, 1084, 0, 0), BeatmapDifficulty::new(0, 0, 0, 0, 10, 952))
        ];

        for beatmap in beatmaps {
            beatmap.insert(db).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Beatmap::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Beatmap {
    Table,
    Id,
    Name,
    Quality,
    Composer,
    Kind,
    DLCPack, // TODO: Add a DLC pack table
    BPMRange,
    Key4,
    Key6,
    Key8,
}

#[derive(Iden, EnumIter)]
pub enum BeatmapKind {
    Tutorial,
    Free,
    Sale,
    DLC,
}
