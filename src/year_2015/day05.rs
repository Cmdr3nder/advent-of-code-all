use anyhow::Result;
use lazy_regex::regex_is_match;

use crate::day::Day;
use crate::input::get_input;

pub struct Day05;

impl Day for Day05 {
    fn main() -> Result<()> {
        let input_str = get_input(2015, 5)?;
        let mut nice_one: usize = 0;
        let mut nice_two: usize = 0;
        for line in input_str.lines() {
            if regex_is_match!(".*[aeiou].*[aeiou].*[aeiou].*", &line)
                && regex_is_match!(
                    "aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz",
                    &line
                )
                && !regex_is_match!("ab|cd|pq|xy", &line)
            {
                nice_one += 1;
            }
            if regex_is_match!("a.a|b.b|c.c|d.d|e.e|f.f|g.g|h.h|i.i|j.j|k.k|l.l|m.m|n.n|o.o|p.p|q.q|r.r|s.s|t.t|u.u|v.v|w.w|x.x|y.y|z.z", &line)
                && regex_is_match!("aa.*aa|ba.*ba|ca.*ca|da.*da|ea.*ea|fa.*fa|ga.*ga|ha.*ha|ia.*ia|ja.*ja|ka.*ka|la.*la|ma.*ma|na.*na|oa.*oa|pa.*pa|qa.*qa|ra.*ra|sa.*sa|ta.*ta|ua.*ua|va.*va|wa.*wa|xa.*xa|ya.*ya|za.*za|ab.*ab|bb.*bb|cb.*cb|db.*db|eb.*eb|fb.*fb|gb.*gb|hb.*hb|ib.*ib|jb.*jb|kb.*kb|lb.*lb|mb.*mb|nb.*nb|ob.*ob|pb.*pb|qb.*qb|rb.*rb|sb.*sb|tb.*tb|ub.*ub|vb.*vb|wb.*wb|xb.*xb|yb.*yb|zb.*zb|ac.*ac|bc.*bc|cc.*cc|dc.*dc|ec.*ec|fc.*fc|gc.*gc|hc.*hc|ic.*ic|jc.*jc|kc.*kc|lc.*lc|mc.*mc|nc.*nc|oc.*oc|pc.*pc|qc.*qc|rc.*rc|sc.*sc|tc.*tc|uc.*uc|vc.*vc|wc.*wc|xc.*xc|yc.*yc|zc.*zc|ad.*ad|bd.*bd|cd.*cd|dd.*dd|ed.*ed|fd.*fd|gd.*gd|hd.*hd|id.*id|jd.*jd|kd.*kd|ld.*ld|md.*md|nd.*nd|od.*od|pd.*pd|qd.*qd|rd.*rd|sd.*sd|td.*td|ud.*ud|vd.*vd|wd.*wd|xd.*xd|yd.*yd|zd.*zd|ae.*ae|be.*be|ce.*ce|de.*de|ee.*ee|fe.*fe|ge.*ge|he.*he|ie.*ie|je.*je|ke.*ke|le.*le|me.*me|ne.*ne|oe.*oe|pe.*pe|qe.*qe|re.*re|se.*se|te.*te|ue.*ue|ve.*ve|we.*we|xe.*xe|ye.*ye|ze.*ze|af.*af|bf.*bf|cf.*cf|df.*df|ef.*ef|ff.*ff|gf.*gf|hf.*hf|if.*if|jf.*jf|kf.*kf|lf.*lf|mf.*mf|nf.*nf|of.*of|pf.*pf|qf.*qf|rf.*rf|sf.*sf|tf.*tf|uf.*uf|vf.*vf|wf.*wf|xf.*xf|yf.*yf|zf.*zf|ag.*ag|bg.*bg|cg.*cg|dg.*dg|eg.*eg|fg.*fg|gg.*gg|hg.*hg|ig.*ig|jg.*jg|kg.*kg|lg.*lg|mg.*mg|ng.*ng|og.*og|pg.*pg|qg.*qg|rg.*rg|sg.*sg|tg.*tg|ug.*ug|vg.*vg|wg.*wg|xg.*xg|yg.*yg|zg.*zg|ah.*ah|bh.*bh|ch.*ch|dh.*dh|eh.*eh|fh.*fh|gh.*gh|hh.*hh|ih.*ih|jh.*jh|kh.*kh|lh.*lh|mh.*mh|nh.*nh|oh.*oh|ph.*ph|qh.*qh|rh.*rh|sh.*sh|th.*th|uh.*uh|vh.*vh|wh.*wh|xh.*xh|yh.*yh|zh.*zh|ai.*ai|bi.*bi|ci.*ci|di.*di|ei.*ei|fi.*fi|gi.*gi|hi.*hi|ii.*ii|ji.*ji|ki.*ki|li.*li|mi.*mi|ni.*ni|oi.*oi|pi.*pi|qi.*qi|ri.*ri|si.*si|ti.*ti|ui.*ui|vi.*vi|wi.*wi|xi.*xi|yi.*yi|zi.*zi|aj.*aj|bj.*bj|cj.*cj|dj.*dj|ej.*ej|fj.*fj|gj.*gj|hj.*hj|ij.*ij|jj.*jj|kj.*kj|lj.*lj|mj.*mj|nj.*nj|oj.*oj|pj.*pj|qj.*qj|rj.*rj|sj.*sj|tj.*tj|uj.*uj|vj.*vj|wj.*wj|xj.*xj|yj.*yj|zj.*zj|ak.*ak|bk.*bk|ck.*ck|dk.*dk|ek.*ek|fk.*fk|gk.*gk|hk.*hk|ik.*ik|jk.*jk|kk.*kk|lk.*lk|mk.*mk|nk.*nk|ok.*ok|pk.*pk|qk.*qk|rk.*rk|sk.*sk|tk.*tk|uk.*uk|vk.*vk|wk.*wk|xk.*xk|yk.*yk|zk.*zk|al.*al|bl.*bl|cl.*cl|dl.*dl|el.*el|fl.*fl|gl.*gl|hl.*hl|il.*il|jl.*jl|kl.*kl|ll.*ll|ml.*ml|nl.*nl|ol.*ol|pl.*pl|ql.*ql|rl.*rl|sl.*sl|tl.*tl|ul.*ul|vl.*vl|wl.*wl|xl.*xl|yl.*yl|zl.*zl|am.*am|bm.*bm|cm.*cm|dm.*dm|em.*em|fm.*fm|gm.*gm|hm.*hm|im.*im|jm.*jm|km.*km|lm.*lm|mm.*mm|nm.*nm|om.*om|pm.*pm|qm.*qm|rm.*rm|sm.*sm|tm.*tm|um.*um|vm.*vm|wm.*wm|xm.*xm|ym.*ym|zm.*zm|an.*an|bn.*bn|cn.*cn|dn.*dn|en.*en|fn.*fn|gn.*gn|hn.*hn|in.*in|jn.*jn|kn.*kn|ln.*ln|mn.*mn|nn.*nn|on.*on|pn.*pn|qn.*qn|rn.*rn|sn.*sn|tn.*tn|un.*un|vn.*vn|wn.*wn|xn.*xn|yn.*yn|zn.*zn|ao.*ao|bo.*bo|co.*co|do.*do|eo.*eo|fo.*fo|go.*go|ho.*ho|io.*io|jo.*jo|ko.*ko|lo.*lo|mo.*mo|no.*no|oo.*oo|po.*po|qo.*qo|ro.*ro|so.*so|to.*to|uo.*uo|vo.*vo|wo.*wo|xo.*xo|yo.*yo|zo.*zo|ap.*ap|bp.*bp|cp.*cp|dp.*dp|ep.*ep|fp.*fp|gp.*gp|hp.*hp|ip.*ip|jp.*jp|kp.*kp|lp.*lp|mp.*mp|np.*np|op.*op|pp.*pp|qp.*qp|rp.*rp|sp.*sp|tp.*tp|up.*up|vp.*vp|wp.*wp|xp.*xp|yp.*yp|zp.*zp|aq.*aq|bq.*bq|cq.*cq|dq.*dq|eq.*eq|fq.*fq|gq.*gq|hq.*hq|iq.*iq|jq.*jq|kq.*kq|lq.*lq|mq.*mq|nq.*nq|oq.*oq|pq.*pq|qq.*qq|rq.*rq|sq.*sq|tq.*tq|uq.*uq|vq.*vq|wq.*wq|xq.*xq|yq.*yq|zq.*zq|ar.*ar|br.*br|cr.*cr|dr.*dr|er.*er|fr.*fr|gr.*gr|hr.*hr|ir.*ir|jr.*jr|kr.*kr|lr.*lr|mr.*mr|nr.*nr|or.*or|pr.*pr|qr.*qr|rr.*rr|sr.*sr|tr.*tr|ur.*ur|vr.*vr|wr.*wr|xr.*xr|yr.*yr|zr.*zr|as.*as|bs.*bs|cs.*cs|ds.*ds|es.*es|fs.*fs|gs.*gs|hs.*hs|is.*is|js.*js|ks.*ks|ls.*ls|ms.*ms|ns.*ns|os.*os|ps.*ps|qs.*qs|rs.*rs|ss.*ss|ts.*ts|us.*us|vs.*vs|ws.*ws|xs.*xs|ys.*ys|zs.*zs|at.*at|bt.*bt|ct.*ct|dt.*dt|et.*et|ft.*ft|gt.*gt|ht.*ht|it.*it|jt.*jt|kt.*kt|lt.*lt|mt.*mt|nt.*nt|ot.*ot|pt.*pt|qt.*qt|rt.*rt|st.*st|tt.*tt|ut.*ut|vt.*vt|wt.*wt|xt.*xt|yt.*yt|zt.*zt|au.*au|bu.*bu|cu.*cu|du.*du|eu.*eu|fu.*fu|gu.*gu|hu.*hu|iu.*iu|ju.*ju|ku.*ku|lu.*lu|mu.*mu|nu.*nu|ou.*ou|pu.*pu|qu.*qu|ru.*ru|su.*su|tu.*tu|uu.*uu|vu.*vu|wu.*wu|xu.*xu|yu.*yu|zu.*zu|av.*av|bv.*bv|cv.*cv|dv.*dv|ev.*ev|fv.*fv|gv.*gv|hv.*hv|iv.*iv|jv.*jv|kv.*kv|lv.*lv|mv.*mv|nv.*nv|ov.*ov|pv.*pv|qv.*qv|rv.*rv|sv.*sv|tv.*tv|uv.*uv|vv.*vv|wv.*wv|xv.*xv|yv.*yv|zv.*zv|aw.*aw|bw.*bw|cw.*cw|dw.*dw|ew.*ew|fw.*fw|gw.*gw|hw.*hw|iw.*iw|jw.*jw|kw.*kw|lw.*lw|mw.*mw|nw.*nw|ow.*ow|pw.*pw|qw.*qw|rw.*rw|sw.*sw|tw.*tw|uw.*uw|vw.*vw|ww.*ww|xw.*xw|yw.*yw|zw.*zw|ax.*ax|bx.*bx|cx.*cx|dx.*dx|ex.*ex|fx.*fx|gx.*gx|hx.*hx|ix.*ix|jx.*jx|kx.*kx|lx.*lx|mx.*mx|nx.*nx|ox.*ox|px.*px|qx.*qx|rx.*rx|sx.*sx|tx.*tx|ux.*ux|vx.*vx|wx.*wx|xx.*xx|yx.*yx|zx.*zx|ay.*ay|by.*by|cy.*cy|dy.*dy|ey.*ey|fy.*fy|gy.*gy|hy.*hy|iy.*iy|jy.*jy|ky.*ky|ly.*ly|my.*my|ny.*ny|oy.*oy|py.*py|qy.*qy|ry.*ry|sy.*sy|ty.*ty|uy.*uy|vy.*vy|wy.*wy|xy.*xy|yy.*yy|zy.*zy|az.*az|bz.*bz|cz.*cz|dz.*dz|ez.*ez|fz.*fz|gz.*gz|hz.*hz|iz.*iz|jz.*jz|kz.*kz|lz.*lz|mz.*mz|nz.*nz|oz.*oz|pz.*pz|qz.*qz|rz.*rz|sz.*sz|tz.*tz|uz.*uz|vz.*vz|wz.*wz|xz.*xz|yz.*yz|zz.*zz", &line) {
                nice_two += 1;
            }
        }
        println!("Nice style 1: {}", nice_one);
        println!("Nice style 2: {}", nice_two);
        Ok(())
    }
}
